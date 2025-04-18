use std::error::Error;
use crate::decode::decode_search;
use crate::request::constants::SPRING_TERM;
use crate::request::search;

mod request {
    pub mod constants;
    pub mod search;
    pub mod index;
    pub mod errors;
}

mod decode {
    mod html;
    mod models;
    mod selectors;
    pub mod errors;
    
    pub use html::decode_search;
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    let subject = "CSC";
    let course_number = 297;
    
    let resp = search::get_course(subject, course_number, SPRING_TERM).await?;
    
    let courses = decode_search(resp)?;
    
    println!("{}", serde_json::to_string_pretty(&courses)?);
    
    Ok(())
}