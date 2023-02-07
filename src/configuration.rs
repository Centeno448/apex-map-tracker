use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::mysql::{MySqlConnectOptions, MySqlSslMode};

#[derive(Debug, serde::Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

#[derive(Debug, serde::Deserialize)]
pub struct ApplicationSettings {
    pub api_key: String,
    pub discord_bot_key: String,
    pub base_log_path: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub name: String,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn without_db(&self) -> MySqlConnectOptions {
        let ssl_mode = if self.require_ssl {
            MySqlSslMode::Required
        } else {
            MySqlSslMode::Preferred
        };
        MySqlConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
            .ssl_mode(ssl_mode)
    }

    pub fn with_db(&self) -> MySqlConnectOptions {
        self.without_db().database(&self.name)
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let builder = config::Config::builder()
        .add_source(config::Environment::with_prefix("AMT").separator("__"));

    builder.build()?.try_deserialize()
}
