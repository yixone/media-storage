use actix_web::web;

pub mod v0;

/// Configures server endpoints
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(v0::config);
}
