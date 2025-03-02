use std::fmt;
use std::fmt::{Display, Formatter};
use serde::Deserialize;
use serde_json::Value;
use crate::request::constants::{client, SEARCH_URL};
use crate::request::errors::RequestError;

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub html: String,
    pub json: Value
}

impl Display for SearchResponse {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.html)
    }
}

/// Returns all courses for a specific subject (string response).
pub async fn get_subject(subject: &str, term: u32) -> Result<SearchResponse, RequestError> {
    request(subject, None, term).await
}

/// Get the course information for a specific course (string response).
pub async fn get_course(
    subject: &str,
    course_number: u32,
    term: u32,
) -> Result<SearchResponse, RequestError> {
    request(subject, Some(course_number), term).await
}

async fn request(
    subject: &str,
    course_number: Option<u32>,
    term: u32,
) -> Result<SearchResponse, RequestError> {
    let term = term.to_string();

    let mut params = vec![("subject", subject), ("term", &term)];

    let course_number = course_number.map(|v| v.to_string());

    if let Some(ref course_number) = course_number {
        params.push(("course-number", course_number));
        params.push(("course-inequality", "="));
    }

    let response = client
        .post(&*SEARCH_URL)
        .form(&params)
        .header("Accept", "application/json")
        .send()
        .await?;

    let success = response.status().is_success();
    let content = response.text().await?;

    if !success {
        return Err(RequestError::UnknownError(content));
    }

    if content.contains("Error: No Data Returned") {
        Err(RequestError::NoCourseFound)
    } else if content.contains("text-warning") {
        Err(RequestError::UnknownError(content))
    } else {
        Ok(serde_json::from_str(&content)?)
    }
}
