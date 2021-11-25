use std::fs;
use serde::{Serialize, Deserialize};


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Github {
    token: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    github: Github
}

impl Config {
    pub fn load(path: &str) -> Config {
        let contents = fs::read_to_string(path)
        .expect("Failed to read file");
        let cfg: Config = serde_yaml::from_str(&contents).unwrap();
        cfg
    }
}
