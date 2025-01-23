use dioxus_logger::tracing;

pub async fn get<T>(url: &str) -> T
where
    T: Default + serde::de::DeserializeOwned,
{
    match reqwest::get(format!(
        "{}/api{}",
        option_env!("BASE_URL").unwrap_or("https://voice-korea.dev.biyard.co/"),
        url
    ))
    .await
    {
        Ok(res) => match res.json::<T>().await {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("Failed to parse {}: {e:?}", url);
                T::default()
            }
        },
        Err(e) => {
            tracing::error!("Failed to get {}: {e:?}", url);
            T::default()
        }
    }
}

use reqwest::{Client, RequestBuilder};
//FIXME: Rename
pub struct ReqwestClient {
    base_url: String,
    pub client: Client,
}

impl ReqwestClient {
    pub fn new() -> Result<Self, reqwest::Error> {
        let base_url = option_env!("API_URL")
            .expect("\"API URL\" Not found")
            .to_string();

        let client = Client::builder().build()?;

        Ok(Self { client, base_url })
    }

    pub fn get(&self, endpoint: &str) -> RequestBuilder {
        self.client
            .get(format!("{}{}", self.base_url.clone(), endpoint))
    }

    pub fn post(&self, endpoint: &str) -> RequestBuilder {
        self.client
            .post(format!("{}{}", self.base_url.clone(), endpoint))
    }

    pub fn put(&self, endpoint: &str) -> RequestBuilder {
        self.client
            .put(format!("{}{}", self.base_url.clone(), endpoint))
    }
    pub fn patch(&self, endpoint: &str) -> RequestBuilder {
        self.client
            .patch(format!("{}{}", self.base_url.clone(), endpoint))
    }
}
