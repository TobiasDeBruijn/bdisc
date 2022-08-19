use std::io;
use actix_web::{App, HttpServer, web};
use dal::fs::Config;
use dal::mysql::Mysql;
use crate::data::AppData;
use crate::routable::Routable;

mod error;
mod data;
mod routes;
mod token;
mod empty;
mod routable;

pub async fn start_actix(config: Config, mysql: Mysql) -> io::Result<()> {
    let data = AppData {
        mysql,
        config
    };
    let appdata = web::Data::new(data);

    HttpServer::new(move || App::new()
        .app_data(appdata.clone())
        .wrap(actix_cors::Cors::permissive())
        .wrap(tracing_actix_web::TracingLogger::default())
        .configure(routes::Router::configure)
    )
    .bind("0.0.0.0:8080")?
    .run()
    .await
}