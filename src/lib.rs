//! This is the library component of the GraphQL API Tester.

pub mod config;
pub mod test_logging;

use std::net::TcpStream;
use test_logging::{TestLogger, TestStatus, LogLevel};

pub fn run_test_case<S: Into<String>>(
    input: &str,
    output: &str,
    address: S
) -> TestLogger {
    let mut logger = TestLogger::new();

    let address = address.into();
    let mut socket = match TcpStream::connect(&address) {
        Ok(val) => val,
        Err(_) => {
            logger.error(format!("Could not connect to server at {address}"));
            logger.mark_fail();
            return logger;
        }
    };
    logger.info(format!("Connected to server at {address}"));

    // TODO: Don't just skip all
    logger.mark_skipped();
    logger
}
