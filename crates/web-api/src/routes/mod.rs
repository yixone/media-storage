use actix_web::{HttpResponse, web};

pub mod v1;

pub mod server_info;

type ApiResult = asset_shelf_result::error::AppResult<HttpResponse>;

/// Configures server endpoints
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(server_info::get_server_info);

    cfg.configure(v1::config);
}
