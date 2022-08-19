use actix_web::web;
use actix_web::web::ServiceConfig;
use crate::routable::Routable;

mod login;
mod callback;

pub struct Router;

impl Routable for Router {
    fn configure(cfg: &mut ServiceConfig) {
        cfg.service(web::scope("/oauth2")
            .route("/callback", web::get().to(callback::callback))
            .route("/login", web::post().to(login::login))
        );
    }
}