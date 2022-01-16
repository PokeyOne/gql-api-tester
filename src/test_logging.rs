#[cfg(test)]
mod tests;

/// A log object to keep track of logged messages at various levels.
pub struct TestLogger {
    logs: Vec<LogElement>,
    test_result: Option<TestStatus>
}

struct LogElement {
    level: LogLevel,
    message: String
}

/// The level and importance of a log.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub enum LogLevel {
    /// Granular logs that are mostly for debuggin only.
    Debug = 0,
    /// General information, but not as bloated as debug.
    Info = 1,
    /// Warnings that should be considered but are not fatal to the program.
    Warning = 2,
    /// A fatal error in the program occured. Most important.
    Error = 3
}

impl LogLevel {
    pub fn name(&self) -> &str {
        match self {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARN",
            LogLevel::Error => "ERROR"
        }
    }
}

pub enum TestStatus {
    Success,
    Failure,
    Skipped
}

impl LogElement {
    pub fn new<S: Into<String>>(level: LogLevel, message: S) -> Self {
        let message = message.into();

        LogElement { level, message }
    }

    pub fn debug<S: Into<String>>(message: S) -> Self {
        Self::new(LogLevel::Debug, message)
    }

    pub fn info<S: Into<String>>(message: S) -> Self {
        Self::new(LogLevel::Info, message)
    }

    pub fn warn<S: Into<String>>(message: S) -> Self {
        Self::new(LogLevel::Warning, message)
    }

    pub fn error<S: Into<String>>(message: S) -> Self {
        Self::new(LogLevel::Error, message)
    }
}

impl TestLogger {
    pub fn new() -> Self {
        TestLogger {
            logs: Vec::new(),
            test_result: None
        }
    }

    pub fn mark_status(&mut self, status: TestStatus) {
        self.test_result = Some(status);
    }

    /// A utility method to set the status to fail
    pub fn mark_fail(&mut self) {
        self.mark_status(TestStatus::Failure);
    }

    /// A utility method to set the status to success
    pub fn mark_success(&mut self) {
        self.mark_status(TestStatus::Success);
    }

    /// A utility method to set the status to skipped
    pub fn mark_skipped(&mut self) {
        self.mark_status(TestStatus::Skipped);
    }

    pub fn log<S: Into<String>>(&mut self, level: LogLevel, message: S) {
        self.logs.push(
            LogElement::new(level, message)
        );
    }

    pub fn debug<S: Into<String>>(&mut self, message: S) {
        self.log(LogLevel::Debug, message)
    }

    pub fn info<S: Into<String>>(&mut self, message: S) {
        self.log(LogLevel::Info, message)
    }

    pub fn warn<S: Into<String>>(&mut self, message: S) {
        self.log(LogLevel::Warning, message)
    }

    pub fn error<S: Into<String>>(&mut self, message: S) {
        self.log(LogLevel::Error, message)
    }

    pub fn flush(&mut self, min_level: LogLevel) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();

        // Push all applicable logs to the result
        for log in &self.logs {
            if log.level >= min_level {
                res.push(format!(
                    "[{}] {}",
                    log.level.name(),
                    log.message
                ));
            }
        }

        // Clear the logs
        self.logs = Vec::new();

        // Return result
        res
    }
}
