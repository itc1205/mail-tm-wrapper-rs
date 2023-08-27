use std::collections::HashMap;

use super::errors::{ApiResult, ApiError};

use crate::adapters::api::schemas;

const BASE_URL: &str = "https://api.mail.tm";

/// Helper function for creating links to api
fn get_url(api_link: &str) -> String {
    BASE_URL.to_owned() + api_link
}

pub(crate) async fn get_domains() -> ApiResult<schemas::ListOfDomains> {
    let url = get_url("/domains");

    let client = reqwest::ClientBuilder::new().build()?;
    let response = client.get(url).send().await?;

    Ok(response.json().await?)
}

pub(crate) async fn create_account(email: String, password: String) -> ApiResult<schemas::Account> {
    let url = get_url("/accounts");

    let mut payload = HashMap::new();
    payload.insert("address", email);
    payload.insert("password", password);
    

    let client = reqwest::ClientBuilder::new().build()?;
    let response = client.post(url).json(&payload).send().await?;

    if !response.status().is_success() {
        return Err(response.into());
    }

    let user: schemas::Account = response.json().await?;

    Ok(user)
}

pub(crate) async fn delete_account(email: String, password: String) -> ApiResult<()> {
    
    Ok(())
}