use actix_web::web;

pub mod assets;
pub mod media;

/// Configures `/v0` endpoints
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("assets").configure(assets::config))
        .service(web::scope("media").configure(media::config));
}
