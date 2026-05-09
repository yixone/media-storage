use actix_web::{HttpResponse, get, web};
use ms_shared_models::domains::AssetId;

use crate::{di::DataContext, error::AppResult};

#[get("/{id}")]
pub async fn get_asset_by_id(
    id: web::Path<(AssetId,)>,
    ctx: web::Data<DataContext>,
) -> AppResult<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}
