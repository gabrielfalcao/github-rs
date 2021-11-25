use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Github {
    token: String,
    username: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    github: Github,
}

impl Config {
    pub fn load(path: &str) -> Config {
        let contents = fs::read_to_string(path).expect("Failed to read file");
        let cfg: Config = serde_yaml::from_str(&contents).unwrap();
        cfg
    }
    pub fn github_token(&self) -> String {
        self.github.token.clone()
    }
    pub fn github_username(&self) -> String {
        self.github.username.clone()
    }
}
