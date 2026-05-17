use actix_web::{HttpResponse, get, web};
use asset_shelf_database::traits::{AssetRepositoryExt, MediaRepositoryExt};
use asset_shelf_models::domains::AssetId;
use asset_shelf_result::create_error;

use crate::{di::DataContext, models::v1::AssetApi, routes::ApiResult};

#[get("/{id}")]
pub async fn get_asset_by_id(
    id: web::Path<AssetId>,
    data_ctx: web::Data<DataContext>,
) -> ApiResult {
    let id = id.into_inner();
    let asset = data_ctx
        .db
        .get_asset(&id)
        .await?
        .ok_or(create_error!(NotFound))?;
    let media = data_ctx
        .db
        .get_media(&asset.media_key)
        .await?
        .ok_or(create_error!(MissingAssetMedia))?;

    let response = AssetApi::from_domain(asset, media);
    Ok(HttpResponse::Ok().json(response))
}
