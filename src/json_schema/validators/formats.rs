use serde_json::{Value};
use std::net;
use uuid;
use url;

use super::super::errors;
use super::super::scope;

#[allow(missing_copy_implementations)]
pub struct Ipv4 {
    pub fragment: Vec<String>,
}

impl super::Validator for Ipv4 {
    fn validate(&self, val: &Value, path: &str, _scope: &scope::Scope) -> super::ValidationState {
        let string = nonstrict_process!(val.as_str(), path);

        match string.parse::<net::Ipv4Addr>() {
            Ok(_) => super::ValidationState::new(),
            Err(_) => {
                val_error!(
                    errors::Format {
                        path: path.to_string(),
                        detail: "Wrong IP address".to_string()
                    }
                )
            }
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct Ipv6 {
    pub fragment: Vec<String>,
}

impl super::Validator for Ipv6 {
    fn validate(&self, val: &Value, path: &str, _scope: &scope::Scope) -> super::ValidationState {
        let string = nonstrict_process!(val.as_str(), path);

        match string.parse::<net::Ipv6Addr>() {
            Ok(_) => super::ValidationState::new(),
            Err(_) => {
                val_error!(
                    errors::Format {
                        path: path.to_string(),
                        detail: "Wrong IP address".to_string()
                    }
                )
            }
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct Uuid {
    pub fragment: Vec<String>,
}

impl super::Validator for Uuid {
    fn validate(&self, val: &Value, path: &str, _scope: &scope::Scope) -> super::ValidationState {
        let string = nonstrict_process!(val.as_str(), path);

        match string.parse::<uuid::Uuid>() {
            Ok(_) => super::ValidationState::new(),
            Err(err) => {
                val_error!(
                    errors::Format {
                        path: path.to_string(),
                        detail: format!("Malformed UUID: {:?}", err)
                    }
                )
            }
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct Uri {
    pub fragment: Vec<String>,
}

impl super::Validator for Uri {
    fn validate(&self, val: &Value, path: &str, _scope: &scope::Scope) -> super::ValidationState {
        let string = nonstrict_process!(val.as_str(), path);

        match url::Url::parse(string) {
            Ok(_) => super::ValidationState::new(),
            Err(err) => {
                val_error!(
                    errors::Format {
                        path: path.to_string(),
                        detail: format!("Malformed URI: {}", err)
                    }
                )
            }
        }
    }
}
