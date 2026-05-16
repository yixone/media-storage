use actix_web::web;

pub mod v1;

pub mod about_server;

/// Configures server endpoints
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(about_server::about_server);

    cfg.configure(v1::config);
}
