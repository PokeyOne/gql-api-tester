use std::path::PathBuf;
use clap::{AppSettings, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "gql_tester", version, author, about)]
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
    }
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Test { path, env } => {
            // TODO: Defaults should be calculated by the config
            let path = path.unwrap_or_else(|| PathBuf::from("./gql_tests/"));
            let _env = env.unwrap_or("test".to_string());
            // TODO: env should be used to calculate an environment configuration
            //       to use and pass to the funciton.

            test_command(path);
        }
    }
}

fn test_command(path: PathBuf) {
    println!("test");
}
