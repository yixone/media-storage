use actix_web::{HttpResponse, get, web};
use shelf_database::traits::{AssetRepoExt, MediaRepoExt};
use shelf_shared_models::domains::{AssetError, AssetId, MediaError};

use crate::{di::DataContext, error::AppResult, models::v1::asset::ApiAsset};

#[get("/{id}")]
pub async fn get_asset_by_id(
    id: web::Path<AssetId>,
    ctx: web::Data<DataContext>,
) -> AppResult<HttpResponse> {
    let id = id.into_inner();
    let asset = ctx
        .db
        .get_asset(&id)
        .await?
        .ok_or(AssetError::AssetNotFound)?;
    let media = ctx
        .db
        .get_media(&asset.media)
        .await?
        .ok_or(MediaError::MediaNotFound)?;

    let api_asset = ApiAsset::from_domains(asset, media);

    Ok(HttpResponse::Ok().json(api_asset))
}
