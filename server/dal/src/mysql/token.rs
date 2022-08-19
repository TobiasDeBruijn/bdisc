use mysql_async::prelude::Queryable;
use mysql_async::{params, Row, TxOpts};
use crate::generate_string;
use crate::mysql::{Mysql, MysqlResult};

pub struct ApiToken {
    mysql: Mysql,
    pub token: String,
    pub name: String,
    pub expiry: Option<i64>,
}

impl ApiToken {
    pub async fn new(mysql: Mysql, name: String, expiry: Option<i64>) -> MysqlResult<Self> {
        let mut tx = mysql.start_transaction(TxOpts::default()).await?;
        let token = generate_string(32);
        tx.exec_drop("INSERT INTO api_tokens (token, name, expiry) VALUES (:token, :name, :expiry)", params! {
            "token" => &token,
            "name" => &name,
            "expiry" => expiry,
        }).await?;

        tx.commit().await?;

        Ok(Self {
            mysql,
            token,
            name,
            expiry
        })
    }

    pub async fn get(mysql: Mysql, token: &str) -> MysqlResult<Option<Self>> {
        let mut tx = mysql.start_transaction(TxOpts::default()).await?;
        let row: Row = match tx.exec_first("SELECT name,expiry FROM api_tokens WHERE token = :token", params! {
            "token" => token
        }).await? {
            Some(x) => x,
            None => return Ok(None)
        };

        let expiry: Option<i64> = row.get("expiry").unwrap();
        if let Some(expiry) = expiry {
            if time::OffsetDateTime::now_utc().unix_timestamp() > expiry {
                Self::delete_inner(mysql.clone(), token).await?;
                return Ok(None);
            }
        }

        Ok(Some(Self {
            mysql,
            token: token.to_string(),
            name: row.get("name").unwrap(),
            expiry,
        }))
    }

    pub async fn delete(&self) -> MysqlResult<()> {
        Self::delete_inner(self.mysql.clone(), &self.token).await
    }

    async fn delete_inner(mysql: Mysql, token: &str) -> MysqlResult<()> {
        let mut tx = mysql.start_transaction(TxOpts::default()).await?;
        tx.exec_drop("DELETE FROM api_tokens WHERE token = :token", params! {
            "token" => token
        }).await?;

        tx.commit().await?;
        Ok(())
    }
}