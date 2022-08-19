use actix_web::http::StatusCode;
use actix_web::ResponseError;
use thiserror::Error;
use discord::DiscordError;

pub type WebResult<T> = Result<T, WebError>;

#[derive(Debug, Error)]
#[allow(unused)]
pub enum WebError {
    #[error("Database error: {0}")]
    Mysql(#[from] dal::mysql::MysqlError),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("Not Found: {0}")]
    NotFound(String),
    #[error("Discord API Error: {0}")]
    DiscordApi(#[from] discord::DiscordError),
}

impl ResponseError for WebError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Mysql(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::DiscordApi(e) => match e {
                DiscordError::Unauthorized => StatusCode::PRECONDITION_FAILED,
                DiscordError::Forbidden => StatusCode::PRECONDITION_FAILED,
                DiscordError::RateLimit => StatusCode::TOO_MANY_REQUESTS,
                _ => StatusCode::BAD_GATEWAY,
            }
        }
    }
}