use actix_web::web;
use dal::fs::Config;
use dal::mysql::Mysql;

pub type WebData = web::Data<AppData>;

#[derive(Debug, Clone)]
pub struct AppData {
    pub mysql: Mysql,
    pub config: Config,
}
