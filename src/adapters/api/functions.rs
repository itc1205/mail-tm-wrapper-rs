use crate::adapters::api::schemas;

const BASE_URL: &str = "https://api.mail.tm";

pub(crate) async fn get_domains() -> Result<schemas::ListOfDomains, reqwest::Error> {
    let mut url = BASE_URL.to_owned();
    url.push_str("/domains");

    let client = reqwest::ClientBuilder::new().build()?;
    let response = client.get(url).send().await?;

    response.json().await
}
