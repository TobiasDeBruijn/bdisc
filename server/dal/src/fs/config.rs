use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::fs::FsResult;
use tracing::info;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    pub discord: DiscordConfig,
    pub mysql: MysqlConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MysqlConfig {
    pub host: String,
    pub db: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub base_url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DiscordConfig {
    pub client_id: String,
    pub client_secret: String,
}

impl Config {
    pub async fn new() -> FsResult<Self> {
        let fpath = Self::path().join("config.toml");
        if !fpath.exists() {
            Self::create_new().await
        } else {
            Self::read_from_disk().await
        }
    }

    async fn read_from_disk() -> FsResult<Self> {
        let mut file = fs::File::open(&Self::path().join("config.toml")).await?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).await?;

        let this: Self = toml::de::from_slice(&buf)?;
        Ok(this)
    }

    async fn create_new() -> FsResult<Self> {
        fs::create_dir_all(&Self::path()).await?;
        let mut file = fs::File::create(&Self::path().join("config.toml")).await?;

        let this = Self::default();
        let ser = toml::ser::to_string_pretty(&this)?;
        file.write_all(&ser.as_bytes()).await?;

        info!("New confiuration has been created. Exit the program and populate it.");

        Ok(this)
    }

    fn path() -> PathBuf {
        #[cfg(not(debug_assertions))]
        let ret = PathBuf::from("/etc/bdisc/");
        #[cfg(debug_assertions)]
        let ret = PathBuf::from("./");

        ret
    }
}