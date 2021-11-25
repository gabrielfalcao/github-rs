use reqwest;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Client {
    token: String,
    client: reqwest::blocking::Client,
}

impl Client {
    pub fn new(token: &str) -> Client {
        let client = reqwest::blocking::Client::new();
        Client {token: token.to_string(), client: client}
    }
}
