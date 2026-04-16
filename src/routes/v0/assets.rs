use actix_multipart::Multipart;
use actix_web::{HttpResponse, get, post, web};

use crate::{
    di::{DataContext, NotificationsContext},
    error::Result,
};

/// Configures `/assets` endpoints
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("assets").service(upload_asset));
}

/// Uploads new Asset and returns it
#[post("")]
pub async fn upload_asset(
    mut payload: Multipart,
    ctx: web::Data<DataContext>,
    nt: web::Data<NotificationsContext>,
) -> Result<HttpResponse> {
    todo!()
}

/// Returns an Asset by ID
#[get("/{id}")]
pub async fn get_asset_by_id(ctx: web::Data<DataContext>) -> Result<HttpResponse> {
    todo!()
}

/// Returns a list of Assets with pagination
#[get("")]
pub async fn get_assets_list(ctx: web::Data<DataContext>) -> Result<HttpResponse> {
    todo!()
}
