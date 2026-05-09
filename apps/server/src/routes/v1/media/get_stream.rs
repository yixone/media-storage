use actix_web::{HttpResponse, get, web};
use ms_shared_models::domains::MediaId;

use crate::{di::DataContext, error::AppResult};

#[get("/{id}")]
pub async fn get_media_stream(
    path: web::Path<(MediaId,)>,
    ctx: web::Data<DataContext>,
) -> AppResult<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}
