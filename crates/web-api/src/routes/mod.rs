use actix_web::web;

pub mod v1;

pub mod server_info;

/// Configures server endpoints
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(server_info::get_server_info);

    cfg.configure(v1::config);
}
