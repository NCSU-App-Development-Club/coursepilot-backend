use thiserror::Error;
use crate::request::search::SearchResponse;

#[derive(Debug, Error)]
pub enum HtmlDecodeError {
    #[error("Malformed course found when decoding SearchResponse: {0}")]
    MalformedCourse(SearchResponse)
}