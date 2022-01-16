use super::*;

fn create_test_logger() -> TestLogger {
    let mut logger = TestLogger::new();

    logger.debug("a");
    logger.info("b");
    logger.warn("c");
    logger.debug("d");
    logger.error("e");

    logger
}

#[test]
fn test_logging_all_levels() {
    let mut logger = create_test_logger();

    let messages = logger.flush(LogLevel::Debug);

    assert_eq!(0, logger.message_count(), "flushing should remove all messages");
    assert_eq!(5, messages.len(), "all messages should be flushed");

    let expected_messages: Vec<String> = vec![
        "[DEBUG] a",
        "[INFO] b",
        "[WARN] c",
        "[DEBUG] d",
        "[ERROR] e"
    ].into_iter()
        .map(|s| s.to_string())
        .collect();

    assert_eq!(messages, expected_messages);
}

#[test]
fn test_log_level_filtering_info() {
    let mut logger = create_test_logger();
    let messages = logger.flush(LogLevel::Info);

    assert_eq!(0, logger.message_count(), "flushing should remove all messages");
    assert_eq!(3, messages.len(), "all info or above messages should be flushed");
}

#[test]
fn test_log_level_filtering_warning() {
    let mut logger = create_test_logger();
    let messages = logger.flush(LogLevel::Warning);

    assert_eq!(0, logger.message_count(), "flushing should remove all messages");
    assert_eq!(2, messages.len(), "all info or above messages should be flushed");
}

#[test]
fn test_log_level_filtering_error() {
    let mut logger = create_test_logger();
    let messages = logger.flush(LogLevel::Error);

    assert_eq!(0, logger.message_count(), "flushing should remove all messages");
    assert_eq!(1, messages.len(), "all info or above messages should be flushed");
}
