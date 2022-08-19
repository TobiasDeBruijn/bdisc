use lazy_static::lazy_static;
use reqwest::Client;

pub mod oauth2;
pub mod user;
mod error;

pub use error::*;

const API_BASE: &str = "https://discord.com/api/v10";

fn path<S: AsRef<str>>(path: S) -> String {
    format!("{API_BASE}{}", path.as_ref())
}

lazy_static! {
    static ref CLIENT: Client = Client::builder()
        .user_agent(format!("BDisc Server v{}", env!("CARGO_PKG_VERSION")))
        .build()
        .unwrap();
}