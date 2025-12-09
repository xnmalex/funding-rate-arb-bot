use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        // Simple example. You can use dotenvy crate if needed.
        let database_url =
            std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://local.db".into());

        Ok(Self { database_url })
    }
}
