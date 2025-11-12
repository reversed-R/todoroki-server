use std::error::Error;

use dotenvy;
use std::env;
use todoroki_use_case::shared::ConfigProvider;

#[derive(Debug, Clone)]
pub struct Config {
    postgres_url: String,
    firebase_project_id: String,
    default_owner_email: String,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        dotenvy::dotenv()?;

        let postgres_user = env::var("POSTGRES_USER")?;
        let postgres_password = env::var("POSTGRES_PASSWORD")?;
        let postgres_hostname = env::var("POSTGRES_HOSTNAME")?;
        let postgres_db = env::var("POSTGRES_DB")?;
        let postgres_port = env::var("POSTGRES_PORT")?;

        let postgres_url = format!(
            "postgresql://{}:{}@{}:{}/{}",
            postgres_user, postgres_password, postgres_hostname, postgres_port, postgres_db
        )
        .to_string();

        let firebase_project_id = env::var("FIREBASE_PROJECT_ID")?;

        let default_owner_email = env::var("APP_DEFAULT_OWNER_EMAIL")?;

        Ok(Self {
            postgres_url,
            firebase_project_id,
            default_owner_email,
        })
    }

    pub fn postgres_url(&self) -> &str {
        &self.postgres_url
    }
}

impl ConfigProvider for Config {
    fn firebase_project_id(&self) -> &str {
        &self.firebase_project_id
    }

    fn default_owner_email(&self) -> &str {
        &self.default_owner_email
    }
}
