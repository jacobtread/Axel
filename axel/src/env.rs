use log::{info, warn};
use std::fmt::Display;
use std::str::FromStr;

/// Initializes the environment by loading the variables from
/// the nearest .env file
pub fn init() {
    dotenv::dotenv().expect("Failed to startup. Unable to load environment from .env");
}

/// Retrieves an environment variable or returns the provided
/// default string instead
pub fn get_env_or(key: &str, default: impl ToString) -> String {
    if let Ok(value) = std::env::var(key) {
        value
    } else {
        default.to_string()
    }
}

/// Retrieves an environment variable or panics if the value
/// is missing
pub fn get_env(key: &str) -> String {
    if let Ok(value) = std::env::var(key) {
        value
    } else {
        panic!("Missing required environment variable: {key}")
    }
}

/// Attempts to retrieve the value with the provided key from
/// the environment variables and parse it as `T` or return
/// the provided default on failure
pub fn parse_env_or<T: FromStr + Display>(key: &str, default: T) -> T {
    if let Ok(value) = std::env::var(key) {
        if let Ok(parsed) = value.parse::<T>() {
            return parsed;
        } else {
            warn!("Unable to parse environment variable '{key}' defaulting to '{default}'");
        }
    }
    default
}

/// Attempts to retrieve the value with the provided key from
/// the environment variables and parse it as `T` or panic
/// on failure
pub fn parse_env<T: FromStr>(key: &str) -> T {
    if let Ok(value) = std::env::var(key) {
        if let Ok(parsed) = value.parse::<T>() {
            return parsed;
        }
    }
    panic!("Missing required environment variable: {key}")
}
