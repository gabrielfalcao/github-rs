use reqwest;
use github_rs_config::Config;
//use std::collections::HashMap;

#[derive(Debug)]
pub struct Client {
    config: Config,
    client: reqwest::blocking::Client,
}

impl Client {
    pub fn new(config: Config) -> Client {
        let client = reqwest::blocking::Client::new();
        Client {config, client}
    }
}
