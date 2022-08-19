use std::ops::Deref;
use mysql_async::{OptsBuilder, Pool};
use thiserror::Error;

mod user;
mod token;
mod oauth2;

pub use user::*;
pub use token::*;
pub use oauth2::*;

#[derive(Debug, Clone)]
pub struct Mysql(Pool);

impl Deref for Mysql {
    type Target = Pool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

mod migrations {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

impl Mysql {
    pub async fn new(host: &str, db: &str, user: &str, passw: &str) -> MysqlResult<Self> {
        let opts = OptsBuilder::default()
            .ip_or_hostname(host)
            .db_name(Some(db))
            .user(Some(user))
            .pass(Some(passw));
        let mut pool = Pool::new(opts);

        migrations::migrations::runner()
            .set_migration_table_name("__bdisc_migrations")
            .run_async(&mut pool)
            .await?;

        Ok(Self(pool))
    }
}

pub type MysqlResult<T> = Result<T, MysqlError>;

#[derive(Debug, Error)]
pub enum MysqlError {
    #[error("{0}")]
    Mysql(#[from] mysql_async::Error),
    #[error("{0}")]
    Refinery(#[from] refinery::Error),
}