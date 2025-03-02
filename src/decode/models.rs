use chrono::{NaiveDate, NaiveTime, Weekday};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Course {
    pub subject: String,
    pub code: u32,
    pub name: String,
    pub description: String,
    pub credits: u8,
    pub sections: Vec<Section>
}

#[derive(Debug, Serialize)]
pub struct Section {
    pub number: u32,
    pub component: String,
    pub class_id: u32,
    pub availability: Availability,
    // schedule is TBD if null
    pub schedule: Option<Schedule>,
    pub location: String,
    pub instructors: Vec<String>,
    pub begin_date: NaiveDate,
    pub end_date: NaiveDate,
    pub notes: Option<String>,
    pub requisites: Option<String>,
    pub restrictions: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Schedule {
    pub days: Vec<Weekday>,
    pub begin_time: NaiveTime,
    pub end_time: NaiveTime,
}

#[derive(Debug, Serialize)]
pub struct Availability {
    pub status: Status, 
    pub capacity: u32,
    pub enrolled: u32,
    pub waitlisted: u32,
}

#[derive(Debug, Serialize)]
pub enum Status {
    Open,
    Closed,
    Waitlisted,
    Reserved,
}

impl TryFrom<&str> for Status {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Open" => Ok(Status::Open),
            "Closed" => Ok(Status::Closed),
            "Waitlisted" => Ok(Status::Waitlisted),
            "Reserved" => Ok(Status::Reserved),
            _ => Err("Invalid status")
        }
    }
}