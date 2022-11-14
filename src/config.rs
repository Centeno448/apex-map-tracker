use std::env;

#[derive(Debug)]
pub struct AppConfig {
    pub api_base_url: String,
    pub api_key: String,
    pub discord_bot_key: String,
    pub map_rotation_url: String,
}

impl AppConfig {
    pub fn default() -> Self {
        let api_base_url =
            env::var("AMC_BASE_API_URL").expect("Could not find AMC_BASE_API_URL in env variables");
        let api_key = env::var("AMC_API_KEY").expect("Could not find AMC_API_KEY in env variables");
        let discord_bot_key = env::var("AMC_DISCORD_BOT_TOKEN")
            .expect("Could not find AMC_DISCORD_BOT_TOKEN in env variables");

        let map_rotation_url = format!("{}/maprotation?auth={}", api_base_url, api_key);

        AppConfig {
            api_base_url,
            api_key,
            discord_bot_key,
            map_rotation_url,
        }
    }
}
