use std::collections::HashMap;

use actix_multipart::{Field, Multipart};
use actix_web::{
    HttpResponse, get, post,
    web::{self, Path},
};
use chrono::Utc;
use futures::TryStreamExt;

use crate::{
    create_error,
    di::DataContext,
    error::{AppError, Result},
    models::{
        api::v0::{ApiAsset, ApiMedia},
        domains::{Asset, AssetId, Media, MediaState},
    },
};

/// Configures `/assets` endpoints
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("assets")
            .service(upload_asset)
            .service(get_asset)
            .service(get_assets_list),
    );
}

#[derive(Default)]
struct AssetUploadingContext {
    media: Option<Media>,
    title: Option<String>,
    source: Option<String>,
    caption: Option<String>,
}

/// Reads a [`actix_multipart::Field`] into a [`String`] or returns an error.
async fn read_field_to_string(field: &mut Field) -> Result<String> {
    let mut buf = Vec::new();
    while let Some(chunk) = field.try_next().await? {
        buf.extend_from_slice(&chunk);
    }
    String::from_utf8(buf).map_err(|_| create_error!(MultipartError))
}

/// Uploads new [`Asset`] and returns it
#[post("")]
pub async fn upload_asset(
    mut payload: Multipart,
    ctx: web::Data<DataContext>,
) -> Result<HttpResponse> {
    let mut uploading = AssetUploadingContext::default();
    while let Some(mut field) = payload.try_next().await? {
        match field.name() {
            Some("file") => {
                if uploading.media.is_some() {
                    continue;
                }

                let disposition = field
                    .content_disposition()
                    .ok_or(create_error!(MultipartError))?;

                if uploading.title.is_none() {
                    uploading.title = disposition.get_filename().map(|v| v.to_string());
                }

                let mimetype = field
                    .content_type()
                    .ok_or(create_error!(MultipartError))?
                    .to_string();

                let res = ctx.store.put_stream(field.map_err(AppError::from)).await?;

                // Get Media from the database or insert a new one
                let media = match res.is_new {
                    true => {
                        let media = Media {
                            id: res.key.into(),
                            state: MediaState::Pending,
                            created_at: Utc::now(),
                            size: res.size as i64,
                            mimetype,
                            width: None,
                            height: None,
                            color: None,
                        };
                        ctx.db.insert_media(&media).await?;
                        media
                    }
                    false => ctx
                        .db
                        .get_media(&res.key.into())
                        .await?
                        .ok_or(create_error!(BrokenRelation))?,
                };

                uploading.media = Some(media);
            }
            Some("title") => uploading.title = Some(read_field_to_string(&mut field).await?),
            Some("source") => uploading.source = Some(read_field_to_string(&mut field).await?),
            Some("caption") => uploading.caption = Some(read_field_to_string(&mut field).await?),
            _ => {
                continue;
            }
        }
    }
    let Some(media) = uploading.media else {
        return Err(create_error!(MultipartError));
    };

    let new_asset = Asset {
        id: AssetId::generate(),
        media: media.id.clone(),
        created_at: Utc::now(),
        title: uploading.title,
        caption: uploading.caption,
        source_url: uploading.source,
    };
    ctx.db.insert_asset(&new_asset).await?;

    let api_media = ApiMedia::from_domain(media);
    let api_asset = ApiAsset::from_domain(new_asset, api_media);
    Ok(HttpResponse::Created().json(api_asset))
}

/// Returns a list of Assets with pagination
#[get("")]
pub async fn get_assets_list(ctx: web::Data<DataContext>) -> Result<HttpResponse> {
    // FIXME: Temporary hardcoded pagination
    let assets = ctx.db.get_assets(0, 50).await?;
    let ids = assets.iter().map(|a| &a.media).collect::<Vec<_>>();

    let media = ctx.db.get_media_by_ids(&ids).await?;
    let media_map = media
        .into_iter()
        .map(|m| (m.id.clone(), ApiMedia::from_domain(m)))
        .collect::<HashMap<_, _>>();

    let api_assets = assets
        .into_iter()
        .map(|a| -> Result<ApiAsset> {
            let media = media_map
                .get(&a.media)
                .cloned()
                .ok_or(create_error!(BrokenRelation))?;
            Ok(ApiAsset::from_domain(a, media))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(HttpResponse::Ok().json(api_assets))
}

/// Returns an [`Asset`] by [`AssetId`]
#[get("/{id}")]
pub async fn get_asset(id: Path<(AssetId,)>, ctx: web::Data<DataContext>) -> Result<HttpResponse> {
    let id = id.into_inner().0;
    let asset = ctx
        .db
        .get_asset(&id)
        .await?
        .ok_or(create_error!(NotFound))?;
    let media = ctx
        .db
        .get_media(&asset.media)
        .await?
        .ok_or(create_error!(BrokenRelation))?;
    let api_media = ApiMedia::from_domain(media);
    let api_asset = ApiAsset::from_domain(asset, api_media);
    Ok(HttpResponse::Ok().json(api_asset))
}
