use std::collections::HashMap;

use actix_web::{HttpResponse, get, web};
use ms_database::{
    pagination::Pagination,
    traits::{AssetRepoExt, MediaRepoExt},
};
use ms_shared_models::domains::MediaError;
use serde::Deserialize;

use crate::{
    di::DataContext,
    error::{AppError, AppResult},
    models::v1::asset::ApiAsset,
};

#[derive(Default, Deserialize)]
struct GetAssetsListQuery {
    offset: Option<u32>,
    limit: Option<u32>,
}

#[get("")]
pub async fn get_assets_list(
    query: web::Query<GetAssetsListQuery>,
    ctx: web::Data<DataContext>,
) -> AppResult<HttpResponse> {
    let pagination = Pagination {
        limit: query.limit.unwrap_or(50),
        offset: query.offset.unwrap_or(0),
    };

    let assets = ctx.db.get_assets(pagination).await?;

    let ids = assets.iter().map(|a| a.media.clone()).collect::<Vec<_>>();
    let media = ctx.db.get_media_by_ids(&ids).await?;

    let media_map = media
        .into_iter()
        .map(|m| (m.id.clone(), m))
        .collect::<HashMap<_, _>>();

    let api_assets = assets
        .into_iter()
        .map(|a| -> AppResult<ApiAsset> {
            let media = media_map
                .get(&a.media)
                .cloned()
                .ok_or(MediaError::MediaNotFound)?;
            Ok(ApiAsset::from_domains(a, media))
        })
        .collect::<Result<Vec<_>, AppError>>()?;

    Ok(HttpResponse::Ok().json(api_assets))
}
