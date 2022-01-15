//! This is the main executable to use from the command line. Code in this part
//! of the code should all be related to the cli, and all actual functionality
//! should be defined under the lib.rs file. This consequently means that most
//! of the `mod` definitions should go in lib.rs and then use them here.

use gql_api_tester::config;

use clap::{Parser, Subcommand};
use config::Config;
use std::path::PathBuf;

const GREEN: &str = "\x1b[92m";
const CLEAR: &str = "\x1b[0m";

#[derive(Parser, Debug)]
#[clap(name = "gql_api_tester", version, author, about)]
struct Cli {
    #[clap(subcommand)]
    command: Command
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Run a particular set of tests or an individual test
    Test {
        /// The directory or file to run tests from. Defaults to "./gql_tests/"
        /// unless specified otherwise in the config file.
        #[clap(parse(from_os_str))]
        path: Option<PathBuf>,
        /// The environment configuration to use. Usually something like
        /// `development`, `test`, or `production`. Defaults to "test" unless
        /// specified otherwise in the config file.
        #[clap(long)]
        env: Option<String>
    },
    /// Interact with the configuration.
    Config {
        #[clap(subcommand)]
        command: ConfigCommand
    }
}

#[derive(Subcommand, Debug)]
enum ConfigCommand {
    /// Load and view the current configuration
    View {
        /// If specified, only show the configuration for the specified
        /// environment. By default all environments are shown.
        #[clap(long)]
        env: Option<String>,
        /// Output the configuration as yaml data. Implies '--full'. Ignores
        /// the '--env' flag.
        #[clap(long)]
        yaml: bool
    }
}

fn main() {
    let args = Cli::parse();
    let (config, warning) = Config::load();

    match args.command {
        Command::Test { path, env } => {
            // TODO: Defaults should be calculated by the config
            let path = path.unwrap_or_else(|| PathBuf::from("./gql_tests/"));
            let _env = env.unwrap_or("test".to_string());
            // TODO: env should be used to calculate an environment configuration
            //       to use and pass to the funciton.

            test_command(path, warning);
        },
        Command::Config { command } => match command {
            ConfigCommand::View { env, yaml } => {
                config_view_command(env, yaml, config, warning);
            }
        }
    }
}

fn test_command(_path: PathBuf, warning: Option<String>) {
    if let Some(warning) = warning {
        println!("{}", warning);
    }

    println!("This command is unimplemented");
}

fn config_view_command(env: Option<String>, yaml: bool, config: Config, warning: Option<String>) {
    if yaml {
        let config_text = config.to_yaml();
        println!("{config_text}");
    } else {
        if let Some(warning) = warning {
            println!("{}", warning);
        }

        let mut max_env_name = 0;

        for env in &config.environments {
            // Update the longest env name
            if env.name.len() > max_env_name {
                max_env_name = env.name.len();
            }
        }

        println!("{:max_env_name$} | endpoint", "env");
        println!("{:-<max_env_name$}-|-{:-<30}", "", "");
        for env in &config.environments {
            let mut name: String = env.name.clone();
            while name.len() < max_env_name {
                name.push(' ');
            }
            let endpoint = config.graphql_endpoint(Some(&name));
            println!("{GREEN}{name}{CLEAR} | {endpoint}");
        }
    }
}
