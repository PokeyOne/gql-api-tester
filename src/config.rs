use std::path::PathBuf;
use serde::{Deserialize, Serialize};

const DEFAULT_GRAPHQL_ENDPOINT: String = "localhost:3000".to_string();

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

        let loaded_config = match serde_yaml::from_str(&config_file_content) {
            Ok(conf) => conf,
            Err(e) => {
                println!("Error in format of config file: {e}");
                return Self::default();
            }
        };

        loaded_config
    }

    /// Seralize the configuration as a Yaml string
    ///
    /// # Panics
    ///
    /// This will only panic if the [`serde_yaml::to_string`] method errors,
    /// which should only error if the Serializer errors. The Serializer
    /// should really never return an error because it is derived. Because this
    /// should never realistically be expected to error, this has intentionally
    /// been left as a panic instead of adding the whole Result overhead.
    pub fn to_yaml(&self) -> String {
        match serde_yaml::to_string(self) {
            Ok(res) => res,
            Err(e) => {
                panic!("Could not convert loaded configuration to yaml: {e}");
            }
        }
    }

    pub fn environment(&self, name: String) -> Option<Environment> {
        for environment in self.environments {
            if &environment.name == &name {
                return Some(environment.clone());
            }
        }

        None
    }

    pub fn graphql_endpoint(&self, env: Option<String>) -> String {
        let env = match env {
            Some(env) => Some(env),
            None => self.default_environment
        };

        // TODO: This should be converted to a let chain once Rust eRFC 2497
        //       is done. See: https://github.com/rust-lang/rust/issues/53667
        if let Some(env) = env &&
            let Some(env) = self.environment(env) &&
            let Some(endpoint) = env.graphql_endpoint
        {
            endpoint
        } else {
            DEFAULT_GRAPHQL_ENDPOINT
        }
    }
}
