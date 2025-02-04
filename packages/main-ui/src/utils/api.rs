use reqwest::{Client, RequestBuilder};

pub struct ReqwestClient {
    base_url: &'static str,
    pub client: Client,
}

impl ReqwestClient {
    pub fn new() -> Result<Self, reqwest::Error> {
        let base_url = crate::config::get().api_url;

        let client = Client::builder().build()?;

        Ok(Self { client, base_url })
    }

    pub fn get(&self, endpoint: &str) -> RequestBuilder {
        self.client.get(format!("{}{}", self.base_url, endpoint))
    }

    pub fn post(&self, endpoint: &str) -> RequestBuilder {
        self.client.post(format!("{}{}", self.base_url, endpoint))
    }

    pub fn put(&self, endpoint: &str) -> RequestBuilder {
        self.client.put(format!("{}{}", self.base_url, endpoint))
    }
    pub fn patch(&self, endpoint: &str) -> RequestBuilder {
        self.client.patch(format!("{}{}", self.base_url, endpoint))
    }
}
