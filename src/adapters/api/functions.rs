use std::collections::HashMap;

use super::errors::{ApiError, ApiResult};

use crate::adapters::api::schemas;

const BASE_URL: &str = "https://api.mail.tm";

/// Helper function for creating links to api
fn get_api_url(api_link: &str, id: Option<&str>) -> String {
    let mut url = BASE_URL.to_owned() + api_link;
    if let Some(id) = id {
        url = url + "/" + id;
    }
    url
}

pub(crate) async fn get_domains() -> ApiResult<schemas::ListOfDomains> {
    let url = get_api_url("/domains", None);

    let client = reqwest::ClientBuilder::new().build()?;
    let response = client.get(url).send().await?;

    Ok(response.json().await?)
}

pub(crate) async fn create_account(email: String, password: String) -> ApiResult<schemas::Account> {
    let url = get_api_url("/accounts", None);

    let mut payload = HashMap::new();
    payload.insert("address", email);
    payload.insert("password", password);

    let client = reqwest::ClientBuilder::new().build()?;
    let response = client.post(url).json(&payload).send().await?;

    if !response.status().is_success() {
        return Err(response.into());
    }

    Ok(response.json().await?)
}

pub(crate) async fn get_token(email: String, password: String) -> ApiResult<schemas::Token> {
    let url = get_api_url("/token", None);

    let mut payload = HashMap::new();
    payload.insert("address", email);
    payload.insert("password", password);

    let client = reqwest::ClientBuilder::new().build()?;
    let response = client.post(url).json(&payload).send().await?;

    if !response.status().is_success() {
        return Err(response.into());
    }

    Ok(response.json().await?)
}

pub(crate) async fn get_user_from_token(token: String) -> ApiResult<schemas::Account> {
    let url = get_api_url("/me", None);
    let client = reqwest::ClientBuilder::new().build()?;
    let response = client.get(url).bearer_auth(token).send().await?;

    Ok(response.json().await?)
}

pub(crate) async fn delete_account(id: String, token: String) -> ApiResult<()> {
    let url = get_api_url("/accounts", Some(&id));
    let client = reqwest::ClientBuilder::new().build()?;

    let response = client
        .delete(url)
        .bearer_auth(token)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(response.into());
    }

    Ok(())
}

pub(crate) async fn get_messages(token: String) -> ApiResult<schemas::ListOfMessages> {
    let url = get_api_url("/messages", None);
    let client = reqwest::ClientBuilder::new().build()?;

    let response = client
        .get(url)
        .bearer_auth(token)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(response.into());
    }

    Ok(response.json().await?)
}

pub(crate) async fn get_message(token: String, id: String) -> ApiResult<schemas::Message> {
    let url = get_api_url("/messages", Some(&id));
    let client = reqwest::ClientBuilder::new().build()?;

    let response = client
        .get(url)
        .bearer_auth(token)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(response.into());
    }

    Ok(response.json().await?)
}