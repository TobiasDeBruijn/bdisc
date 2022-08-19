use reqwest::StatusCode;
use thiserror::Error;

pub type DiscordResult<T> = Result<T, DiscordError>;

#[derive(Debug, Error)]
pub enum DiscordError {
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("RateLimit")]
    RateLimit,
}

pub(crate) fn map_reqwest_error_status(error: reqwest::Error) -> DiscordError {
    match error.status().unwrap() {
        StatusCode::UNAUTHORIZED => DiscordError::Unauthorized,
        StatusCode::FORBIDDEN => DiscordError::Forbidden,
        StatusCode::TOO_MANY_REQUESTS => DiscordError::RateLimit,
        _ => DiscordError::Reqwest(error)
    }
}