use github_rs_config::Config;
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;

use reqwest::blocking::RequestBuilder;
#[derive(Debug)]
pub struct Client {
    config: Config,
    client: reqwest::blocking::Client,
}

impl Client {
    pub fn new(config: Config) -> Client {
        let client = reqwest::blocking::Client::new();
        Client { config, client }
    }

    fn wrap_request(&self, req: RequestBuilder) -> RequestBuilder {
        let mut headers = HeaderMap::new();
        let token = self.config.github_token();
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/vnd.github.v3+json"),
        );
        headers.insert("User-Agent", HeaderValue::from_static("github-rs"));
        req.headers(headers).bearer_auth(token)
    }
    pub fn get_user(&self) {
        let response = self
            .wrap_request(self.client.get("https://api.github.com/user"))
            .send()
            .unwrap();
        let data = response.text().unwrap();
        let v: Value = serde_json::from_str(&data).unwrap();
        println!("{}", serde_json::to_string_pretty(&v).unwrap());
    }
}
