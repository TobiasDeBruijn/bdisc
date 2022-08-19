use tracing::Level;
use dal::fs::Config;
use dal::mysql::Mysql;

#[tokio::main]
async fn main() {
    configure_tracing();

    let cfg = Config::new().await.expect("Reading config");
    let mysql = Mysql::new(&cfg.mysql.host, &cfg.mysql.db, &cfg.mysql.user, &cfg.mysql.password).await.expect("Setting up mysql");

    routes::start_actix(cfg, mysql).await.expect("Starting server");
}

fn configure_tracing() {
    let sub = tracing_subscriber::fmt()
        .compact()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(sub).unwrap();
}