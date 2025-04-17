use crate::decode::errors::HtmlDecodeError;
use crate::decode::models::{Availability, Course, Instructor, Schedule, Section, Status};
use crate::decode::selectors::*;
use crate::request::search::SearchResponse;
use chrono::{NaiveDate, NaiveTime, Weekday};
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;

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

// Code below for parsing a single section

// todo: test for multiple professors, only tested for one at the moment
fn parse_instructors(instructor_html: scraper::ElementRef) -> Vec<Instructor> {
    let mut instructors = Vec::new();

    for link in instructor_html.child_elements() {
        instructors.push(Instructor {
            name: link.text().collect::<String>().trim().to_string(),
            webpage: link.value().attr("href").map(|s| s.to_string()),
        })
    }

    instructors
}

fn parse_availability(availability_html: scraper::ElementRef) -> Option<Availability> {
    let mut status_children = availability_html.text();

    let status = status_children.next()?;

    let fullness = status_children.next()?;
    let status = Status::try_from(status).ok()?;

    // regex that matches both "enrolled/capacity" and "enrolled/capacity (waitlisted)"
    lazy_static! {
        static ref re: Regex = Regex::new(r"(\d+)/(\d+)(?:\s*\((\d+)\))?").unwrap();
    }

    let caps = re.captures(fullness)?;

    let enrolled = caps.get(1)?.as_str().parse::<u32>().ok()?;
    let capacity = caps.get(2)?.as_str().parse::<u32>().ok()?;

    let waitlisted = caps.get(3)
        .and_then(|m| m.as_str().parse::<u32>().ok())
        .unwrap_or(0);
    
    Some(Availability {
        status,
        capacity,
        enrolled,
        waitlisted,
    })
}

fn parse_schedule(schedule_html: scraper::ElementRef) -> Option<Schedule> {
    if schedule_html.text().next() == Some("TBD") {
        None
    } else {
        let times = schedule_html.text().last()?.trim();
        let time_pair = times.split(" - ").collect::<Vec<&str>>();

        let begin = NaiveTime::parse_from_str(time_pair.first()?.trim(), "%I:%M %p").ok()?;
        let end = NaiveTime::parse_from_str(time_pair.last()?.trim(), "%I:%M %p").ok()?;

        let days = schedule_html
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
}

fn parse_begin_end_date(date_html: scraper::ElementRef) -> Option<(NaiveDate, NaiveDate)> {
    let date_range = date_html.text().collect::<String>();
    let date_pair = date_range.split(" - ").collect::<Vec<&str>>();
    
    let begin = NaiveDate::parse_from_str(date_pair.first()?.trim(), "%m/%d/%y").ok()?;
    let end = NaiveDate::parse_from_str(date_pair.last()?.trim(), "%m/%d/%y").ok()?;
    
    Some((begin, end))
    
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

    let availability = parse_availability(children.next()?)?;

    let schedule = parse_schedule(children.next()?);
    
    let location = children.next()?.text().collect::<String>();
    
    let instructors = parse_instructors(children.next()?);
    
    let (begin_date, end_date) = parse_begin_end_date(children.next()?)?;
    
    // skip an empty element
    children.next()?;
    
    let misc = children.next()?.select(&LINK_SELECTOR);
    
    let mut notes = None;
    let mut requisites = None;
    let mut restrictions = None;
    
    // note: content may contain raw html
    for link in misc {
        let id = link.value().attr("id");
        let content = link.value().attr("data-content");
        
        if id.is_none() || content.is_none() {
            continue;
        }
        
        let content = content.unwrap().trim().to_string();
        
        if id.unwrap().starts_with("notes") {
            notes = Some(content);
        } else if id.unwrap().starts_with("reqs") {
            requisites = Some(content);
        } else if id.unwrap().starts_with("reserve") {
            restrictions = Some(content);
        }
    }

    Some(Section {
        number: section_number,
        component,
        class_id,
        availability,
        schedule,
        location,
        instructors,
        begin_date,
        end_date,
        notes,
        requisites,
        restrictions,
    })
}