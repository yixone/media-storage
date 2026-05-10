use actix_web::{HttpResponse, patch, web};
use ms_database::traits::AssetRepoExt;
use ms_shared_models::{
    domains::{AssetError, AssetId, AssetPatchData},
    patch::PatchField,
};
use serde::Deserialize;

use crate::{di::DataContext, error::AppResult};

#[derive(Debug, Deserialize)]
struct UpdateAssetRequest {
    title: Option<String>,
    caption: Option<String>,
    source_url: Option<String>,
}

#[patch("/{id}")]
pub async fn update_asset(
    id: web::Path<AssetId>,
    payload: web::Json<UpdateAssetRequest>,
    ctx: web::Data<DataContext>,
) -> AppResult<HttpResponse> {
    let payload = payload.into_inner();

    let patch = AssetPatchData {
        title: PatchField::from_option_string(payload.title),
        caption: PatchField::from_option_string(payload.caption),
        source_url: PatchField::from_option_string(payload.source_url),
    };

    if !ctx.db.patch_asset(&id, &patch).await? {
        return Err(AssetError::AssetNotFound)?;
    }

    Ok(HttpResponse::NoContent().finish())
}
