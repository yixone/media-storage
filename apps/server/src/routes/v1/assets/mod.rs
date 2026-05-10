use actix_web::web;

mod get_by_id;
mod get_list;
mod update;
mod upload;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("assets")
            .service(get_by_id::get_asset_by_id)
            .service(get_list::get_assets_list)
            .service(update::update_asset)
            .service(upload::upload_asset),
    );
}
