#[cfg(test)]
mod tests;

use std::path::PathBuf;
use serde::{Deserialize, Serialize};

const DEFAULT_GRAPHQL_ENDPOINT: &str = "localhost:3000/graphql";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Config {
    pub environments: Vec<Environment>,
    /// The default environment to run. optional. Takes precedence over the
    /// default_graphql_endpoint option if the environment has an enpoint defined.
    pub default_environment: Option<String>,
    pub default_graphql_endpoint: Option<String>
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Environment {
    pub name: String,
    pub graphql_endpoint: Option<String>
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
            default_graphql_endpoint: Some("localhost:3000/graphql".to_string())
        }
    }

    /// Load a configuration file or return a default one. Return result is
    /// a tuple containing the [`Config`] and a optional of a printable warning.
    /// For example if a config can not be found or loaded, there will be a
    /// warning that can be printed to the user. The warning is not printed
    /// directly because certain commands require very specific output format and having the warning printed would defeat those.
    pub fn load() -> (Self, Option<String>) {
        let path = PathBuf::from("./gql_api_tester.yml");

        if !path.exists() {
            let warning = format!("No config file exists in project; loading default.
To add a config either create gql_api_tester.yml in the project root or run:
  $ gql_api_tester config init");

            return (Self::default(), Some(warning));
        }

        let config_file_content: String = match std::fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => {
                let warning = format!("Could not read config file for reason: {e}
Loading default config");
                return (Self::default(), Some(warning));
            }
        };

        let loaded_config = match serde_yaml::from_str(&config_file_content) {
            Ok(conf) => conf,
            Err(e) => {
                let warning = format!("Error in format of config file: {e}");
                return (Self::default(), Some(warning));
            }
        };

        (loaded_config, None)
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

    /// Look for and get the environment with the name given.
    pub fn environment(&self, name: &str) -> Option<Environment> {
        for environment in &self.environments {
            if &environment.name == name {
                return Some(environment.clone());
            }
        }

        None
    }

    pub fn graphql_endpoint(&self, env: Option<&str>) -> String {
        // Get the current environment in the option given, or if none check for
        // a default environment.
        let env_maybe: Option<&str> = match env {
            Some(val) => Some(val),
            None => match &self.default_environment {
                Some(val) => Some(val.as_str()),
                None => None
            }
        };

        // TODO: This should be converted to a let chain once Rust eRFC 2497
        //       is done. See: https://github.com/rust-lang/rust/issues/53667
        //  SEE: b9ede957ca220e5630f774cf9b7851a934393a9e (git commit)

        if let Some(env_name) = env_maybe {
            if let Some(env) = self.environment(env_name) {
                if let Some(endpoint) = env.graphql_endpoint {
                    return endpoint;
                }
            }
        }

        DEFAULT_GRAPHQL_ENDPOINT.to_string()
    }
}
