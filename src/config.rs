pub mod config {
    use serde::{Deserialize, Serialize};
    use std::fs;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    pub struct GithubConfig {
        token: String,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    pub struct Config {
        github: GithubConfig,
    }

    impl Config {
        pub fn load(path: &str) -> Config {
            let contents = fs::read_to_string(path).expect("Failed to read file");
            let cfg: Config = serde_yaml::from_str(&contents).unwrap();
            cfg
        }
    }
}
