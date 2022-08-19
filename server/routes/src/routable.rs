use actix_web::web::ServiceConfig;

pub trait Routable {
    fn configure(cfg: &mut ServiceConfig);
}