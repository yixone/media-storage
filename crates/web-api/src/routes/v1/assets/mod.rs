use actix_web::web;

mod get_by_id;
mod get_list;
mod update;
mod upload;

/// Configures `/asset` endpoints
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("assets")
            .service(get_by_id::get_asset_by_id)
            .service(upload::upload_asset),
    );
}
