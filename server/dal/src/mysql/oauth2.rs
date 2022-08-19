use mysql_async::prelude::Queryable;
use mysql_async::{params, Row, TxOpts};
use crate::generate_string;
use crate::mysql::{Mysql, MysqlResult};

pub struct Oauth2State {
    mysql: Mysql,
    pub state: String,
    pub api_token: String,
}

impl Oauth2State {
    pub async fn new(mysql: Mysql, api_token: String) -> MysqlResult<Self> {
        let mut tx = mysql.start_transaction(TxOpts::default()).await?;
        let state = generate_string(32);
        tx.exec_drop("INSERT INTO oauth2_login_states (state, token) VALUES (:state, :token)", params! {
            "state" => &state,
            "token" => &api_token,
        }).await?;

        tx.commit().await?;
        Ok(Self {
            mysql,
            state,
            api_token,
        })
    }

    pub async fn get(mysql: Mysql, state: &str) -> MysqlResult<Option<Self>> {
        let mut tx = mysql.start_transaction(TxOpts::default()).await?;
        let row: Row = match tx.exec_first("SELECT token FROM oauth2_login_states WHERE state = :state", params! {
            "state" => state
        }).await? {
            Some(x) => x,
            None => return Ok(None)
        };

        Ok(Some(Self {
            mysql,
            state: state.to_string(),
            api_token: row.get("token").unwrap(),
        }))
    }

    pub async fn delete(&self) -> MysqlResult<()> {
        let mut tx = self.mysql.start_transaction(TxOpts::default()).await?;
        tx.exec_drop("DELETE FROM oauth2_login_states WHERE state = :state", params! {
            "state" => &self.state
        }).await?;
        tx.commit().await?;
        Ok(())
    }
}