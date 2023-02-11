use core::fmt;

#[derive(Debug, Clone)]
pub struct CommandError(String);

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Command failed with error: {}", &self.0)
    }
}

impl From<reqwest::Error> for CommandError {
    fn from(value: reqwest::Error) -> Self {
        Self(value.to_string())
    }
}

impl From<sqlx::Error> for CommandError {
    fn from(value: sqlx::Error) -> Self {
        Self(value.to_string())
    }
}
