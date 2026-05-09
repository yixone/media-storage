use actix_web::web;

mod get_by_id;
mod get_list;
mod upload;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("assets")
            .service(upload::upload_asset)
            .service(get_list::get_assets_list)
            .service(get_by_id::get_asset_by_id),
    );
}
