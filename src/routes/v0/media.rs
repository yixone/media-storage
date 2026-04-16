use actix_web::{HttpResponse, get, web};

use crate::{di::DataContext, error::Result};

/// Configures `/media` endpoints
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("media").service(get_media_stream));
}

/// Returns media stream by id
#[get("/{id}")]
pub async fn get_media_stream(ctx: web::Data<DataContext>) -> Result<HttpResponse> {
    todo!()
}
