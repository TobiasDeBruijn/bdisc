use mysql_async::{params, TxOpts};
use mysql_async::prelude::Queryable;
use crate::mysql::{Mysql, MysqlResult};

pub struct User {
    mysql: Mysql,
    pub id: String,
    pub refresh_token: String,
}

impl User {
    pub async fn new(mysql: Mysql, discord_id: String, refresh_token: String) -> MysqlResult<Self> {
        let mut tx = mysql.start_transaction(TxOpts::default()).await?;
        tx.exec_drop("INSERT INTO users (id, refresh_token) VALUES (:id, :refresh_token)", params! {
            "id" => &discord_id,
            "refresh_token" => &refresh_token
        }).await?;

        tx.commit().await?;

        Ok(Self {
            mysql,
            id: discord_id,
            refresh_token
        })
    }

    pub async fn add_access_token(&self, access_token: String, expires_at: i64) -> MysqlResult<()> {
        let mut tx = self.mysql.start_transaction(TxOpts::default()).await?;
        tx.exec_drop("INSERT INTO oauth2_access_tokens (token, user, expiry) VALUES (:token, :user, :expiry)", params! {
            "token" => &access_token,
            "user" => &self.id,
            "expiry" => expires_at
        }).await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn get_access_token(&self) -> MysqlResult<Option<String>> {
        todo!()
    }
}