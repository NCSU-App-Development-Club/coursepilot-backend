use crate::request::constants::{client, INDEX_URL};
use crate::request::errors::RequestError;

/// Returns HTML content of the index page (contains all term IDs, current term, and more)
pub async fn get_index() -> Result<String, RequestError> {
    let response = client.get(&*INDEX_URL)
        .header("Accept", "application/json")
        .send()
        .await?;

    let success = response.status().is_success();
    let content = response.text().await?;

    if !success {
        return Err(RequestError::UnknownError(content));
    }

    Ok(content)
}