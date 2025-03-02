use crate::decode::errors::HtmlDecodeError;
use crate::decode::models::{Availability, Course, Schedule, Section, Status};
use crate::decode::selectors::*;
use crate::request::search::SearchResponse;
use chrono::{NaiveTime, Weekday};
use std::str::FromStr;

/// Decodes the HTML response from a course search into a list of courses.
/// If you used get_course, the response should be a single course.
/// If an error occurs, the function will return an error
pub fn decode_search(search_response: SearchResponse) -> Result<Vec<Course>, HtmlDecodeError> {
    let document = scraper::Html::parse_document(&search_response.html);

    let mut course_list = Vec::new();

    let courses = document.select(&COURSE_SELECTOR);

    for course in courses {
        let course = parse_course(course);

        match course {
            Some(course) => course_list.push(course),
            None => return Err(HtmlDecodeError::MalformedCourse(search_response)),
        }
    }

    Ok(course_list)
}

fn parse_course(course_html: scraper::ElementRef) -> Option<Course> {
    let name_pair = course_html
        .value()
        .attr("id")?
        .split("-")
        .collect::<Vec<&str>>();

    let subject = name_pair.first()?.to_string();
    let code = name_pair.last()?.parse::<u32>().ok()?;

    let name = course_html
        .select(&SMALL_SELECTOR)
        .next()?
        .text()
        .collect::<String>();
    let credits = course_html
        .select(&UNITS_SELECTOR)
        .next()?
        .text()
        .collect::<String>()
        .split(':')
        .last()?
        .trim()
        .parse::<u8>()
        .ok()?;

    // make each paragraph a new line
    let description = course_html
        .select(&PARAGRAPH_SELECTOR)
        .map(|p| p.text().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");

    let mut course = Course {
        subject,
        code,
        name,
        description,
        credits,
        sections: Vec::new(),
    };

    let sections = course_html.select(&SECTION_SELECTOR).skip(2);

    for section in sections {
        let section = parse_section(section);

        match section {
            Some(section) => course.sections.push(section),
            None => return None,
        }
    }

    Some(course)
}

fn parse_section(section_html: scraper::ElementRef) -> Option<Section> {
    let mut children = section_html.child_elements();

    let section_number = children
        .next()?
        .text()
        .collect::<String>()
        .parse::<u32>()
        .ok()?;
    let component = children.next()?.text().collect::<String>();
    let class_id = children
        .next()?
        .text()
        .collect::<String>()
        .parse::<u32>()
        .ok()?;

    let availability = {
        let mut status_children = children.next()?.text();

        let status = status_children.next()?;

        let fullness = status_children.next()?;
        let mut capacity_pair = fullness.split('/');

        let status = Status::try_from(status).ok()?;
        let capacity = capacity_pair.next()?.parse::<u32>().ok()?;
        let enrolled = capacity_pair.next()?.parse::<u32>().ok()?;

        // can't test adding waitlisted until there are courses with waitlisted sections
        // TODO: once enrollment begins, add waitlisted parsing
        let waitlisted = 0_u32;

        Availability {
            status,
            capacity,
            enrolled,
            waitlisted,
        }
    };

    let schedule = {
        let schedule_children = children.next()?;

        if schedule_children.text().next() == Some("TBD") {
            None
        } else {
            let times = schedule_children.text().last()?.trim();
            let time_pair = times.split(" - ").collect::<Vec<&str>>();

            let begin = NaiveTime::parse_from_str(time_pair.first()?.trim(), "%I:%M %p").ok()?;
            let end = NaiveTime::parse_from_str(time_pair.last()?.trim(), "%I:%M %p").ok()?;

            let days = schedule_children
                .select(&ABBREVIATION_SELECTOR)
                .map(|abbr| abbr.value()
                    .attr("title")
                    .unwrap_or(" - meet")  // if the title attribute is missing, force parsing to fail
                     // this is probably bad practice, but it's the best I can do for now
                )
                .filter(|s| s.contains(" - meet"))
                .map(|s| s.replace(" - meet", ""))
                .map(|s| Weekday::from_str(&s))
                .collect::<Result<Vec<_>, _>>();
            
            Some(Schedule {
                days: days.ok()?,
                begin_time: begin,
                end_time: end,
            })
        }
    };
    
    let location = children.next()?.text().collect::<String>();
    
    // TODO: add instructor parsing (w/ support for multiple instructors)
    // TODO: add begin and end date parsing
    // TODO: add notes parsing (class notes, class requisites, and class restrictions)

    Some(Section {
        number: section_number,
        component,
        class_id,
        availability,
        schedule,
        location,
        instructors: todo!(),
        begin_date: todo!(),
        end_date: todo!(),
        notes: todo!(),
        requisites: todo!(),
        restrictions: todo!(),
    })
}
