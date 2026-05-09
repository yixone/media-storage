use actix_web::web;

pub mod assets;
pub mod media;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("v1")
            .configure(assets::config)
            .configure(media::config),
    );
}
