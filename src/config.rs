use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    environments: Vec<Environment>,
    default_environment: Option<String>,
    default_graphql_endpoint: Option<String>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Environment {
    name: String,
    graphql_endpoint: Option<String>
}

impl Environment {
    pub fn new<S: Into<String>>(name: S, graphql_endpoint: Option<String>) -> Self {
        let name = name.into();

        Self { name, graphql_endpoint }
    }
}

impl Config {
    pub fn default() -> Self {
        Config {
            environments: vec![
                Environment::new("test", None),
                Environment::new("development", None),
                Environment::new("production", None)
            ],
            default_environment: Some("development".to_string()),
            default_graphql_endpoint: Some("localhost:3000".to_string())
        }
    }

    pub fn load() -> Self {
        let path = PathBuf::from("./gql_api_tester.yml");

        if !path.exists() {
            println!("No config file exists in project; loading default.
To add a config either create gql_api_tester.yml in the project root or run:
  $ gql_api_tester config init");

            return Self::default();
        }

        let config_file_content: String = match std::fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => {
                println!("Could not read config file for reason: {e}
Loading default config");
                return Self::default();
            }
        };

        let mut loaded_config = match serde_yaml::from_str(&config_file_content) {
            Ok(conf) => conf,
            Err(e) => {
                println!("Error in format of config file: {e}");
                return Self::default();
            }
        };

        loaded_config
    }
}
