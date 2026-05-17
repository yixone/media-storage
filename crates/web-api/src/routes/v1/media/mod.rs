use actix_web::web;

mod get_stream;

/// Configures `/media` endpoints
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("media").service(get_stream::get_media_stream));
}
