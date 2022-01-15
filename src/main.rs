mod config;

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
    let config = Config::load();

    match args.command {
        Command::Test { path, env } => {
            // TODO: Defaults should be calculated by the config
            let path = path.unwrap_or_else(|| PathBuf::from("./gql_tests/"));
            let _env = env.unwrap_or("test".to_string());
            // TODO: env should be used to calculate an environment configuration
            //       to use and pass to the funciton.

            test_command(path);
        },
        Command::Config { command } => match command {
            ConfigCommand::View { env, yaml } => {
                config_view_command(env, yaml, config);
            }
        }
    }
}

fn test_command(_path: PathBuf) {
    println!("This command is unimplemented");
}

fn config_view_command(env: Option<String>, yaml: bool, config: Config) {
    if yaml {
        let config_text = config.to_yaml();
        println!("{config_text}");
    } else {
        println!("{GREEN}--- Environments Loaded ---{CLEAR}");
        for env in &config.environments {
            println!("  {}", env.name);
        }
    }
}
