use serde::Serialize;

use crate::utilities::traits::FormStyle;
use std::vec;

#[derive(Debug, Clone, Serialize)]
pub struct SignupCredentials {
    pub email: String,
    pub name: String,
    pub password: String,
    pub passconf: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

impl FormStyle for SignupCredentials {
    fn fields(&self) -> Vec<String> {
        vec![
            String::from("email"),
            String::from("name"),
            String::from("password"),
            String::from("passconf"),
        ]
    }

    fn set(&mut self, field: &str, value: String) {
        match field {
            "email" => self.email = value,
            "name" => self.name = value,
            "password" => self.password = value,
            "passconf" => self.passconf = value,
            _ => {}
        }
    }

    fn get(&mut self, field: &str) -> Option<String> {
        match field {
            "email" => Some(self.email.clone()),
            "name" => Some(self.name.clone()),
            "password" => Some(self.password.clone()),
            "passconf" => Some(self.passconf.clone()),
            _ => None,
        }
    }
}

impl FormStyle for LoginCredentials {
    fn fields(&self) -> Vec<String> {
        vec![String::from("email"), String::from("password")]
    }

    fn set(&mut self, field: &str, value: String) {
        match field {
            "email" => self.email = value,
            "password" => self.password = value,
            _ => {}
        }
    }

    fn get(&mut self, field: &str) -> Option<String> {
        match field {
            "email" => Some(self.email.clone()),
            "password" => Some(self.password.clone()),
            _ => None,
        }
    }
}

impl Default for SignupCredentials {
    fn default() -> SignupCredentials {
        SignupCredentials {
            email: "".to_string(),
            name: "".to_string(),
            password: "".to_string(),
            passconf: "".to_string(),
        }
    }
}

impl Default for LoginCredentials {
    fn default() -> LoginCredentials {
        LoginCredentials {
            email: "".to_string(),
            password: "".to_string(),
        }
    }
}
