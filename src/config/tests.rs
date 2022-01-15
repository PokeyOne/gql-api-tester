use super::*;

fn create_test_config() -> Config {
    Config {
        environments: vec![
            Environment::new("test", None),
            Environment::new("development", None),
            Environment::new("production", Some("https://example.com/graphql".to_string()))
        ],
        default_environment: Some("development".to_string()),
        default_graphql_endpoint: Some("localhost:3000/graphql".to_string())
    }
}

#[test]
fn test_to_yaml() {
    assert_eq!(
        create_test_config(),
        serde_yaml::from_str(&create_test_config().to_yaml()).unwrap()
    );
}

#[test]
fn test_get_environment() {
    let config = create_test_config();

    let env = config.environment("test").unwrap();
    assert_eq!("test".to_string(), env.name);
}

#[test]
fn test_graphql_endpoint_should_default_when_no_env() {
    assert_eq!(
        "localhost:3000/graphql".to_string(),
        create_test_config().graphql_endpoint(None)
    );
}

#[test]
fn test_graphql_endpoint_should_default_when_no_default_in_env() {
    assert_eq!(
        "localhost:3000/graphql".to_string(),
        create_test_config().graphql_endpoint(Some(&"test"))
    );
}

#[test]
fn test_graphql_endpoint_should_be_from_env() {
    assert_eq!(
        "https://example.com/graphql".to_string(),
        create_test_config().graphql_endpoint(Some(&"production"))
    );
}
