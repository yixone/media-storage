use actix_multipart::{Field, Multipart};
use actix_web::{HttpResponse, get, post, web};
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

/// Reads a [`actix_multipart::Field`] into a string or returns an error.
async fn read_field_to_string(field: &mut Field) -> Result<String> {
    let mut buf = Vec::new();
    while let Some(chunk) = field.try_next().await? {
        buf.extend_from_slice(&chunk);
    }
    String::from_utf8(buf).map_err(|_| create_error!(MultipartError))
}

/// Uploads new Asset and returns it
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
                    // For now we ignore all files except the first one
                    continue;
                }

                // Set the file name as the title if it is empty
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

                // Uploading a file to storage
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
                        // NOTE:
                        // When inserting, we avoid data races because the StorageKey depends on the file hash,
                        // so even if there is an insertion conflict, we will get the same Media
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
            Some("title") => {
                uploading.title = Some(read_field_to_string(&mut field).await?);
            }
            Some("source") => {
                uploading.source = Some(read_field_to_string(&mut field).await?);
            }
            Some("caption") => {
                uploading.caption = Some(read_field_to_string(&mut field).await?);
            }
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
    todo!()
}

/// Returns an Asset by ID
#[get("/{id}")]
pub async fn get_asset(ctx: web::Data<DataContext>) -> Result<HttpResponse> {
    todo!()
}
