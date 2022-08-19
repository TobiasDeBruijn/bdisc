use actix_web::web;
use actix_web::web::ServiceConfig;
use crate::routable::Routable;

mod oauth2;

pub struct Router;

impl Routable for Router {
    fn configure(cfg: &mut ServiceConfig) {
        cfg.service(web::scope("")
            .configure(oauth2::Router::configure)
        );
    }
}