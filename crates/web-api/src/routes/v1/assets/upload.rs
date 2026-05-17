use actix_multipart::Multipart;
use actix_web::{HttpResponse, post, web};
use asset_shelf_database::traits::{AssetRepositoryExt, MediaRepositoryExt};
use asset_shelf_models::domains::{Asset, AssetId, Media, MediaId, MediaStatus};
use asset_shelf_result::create_error;
use chrono::Utc;
use futures::TryStreamExt;
use tokio_util::io::StreamReader;

use crate::{
    bg::media_worker::MediaWorkerTask,
    di::{DataContext, MessagesContext},
    models::v1::AssetApi,
    routes::ApiResult,
    utils::multipart::MultipartParseError,
};

/// Asset uploading context
#[derive(Default)]
struct UploadingContext {
    media: Option<Media>,
    is_new_media: bool,
    title: Option<String>,
    caption: Option<String>,
    source_url: Option<String>,
}

#[post("/upload")]
pub async fn upload_asset(
    mut payload: Multipart,
    data_ctx: web::Data<DataContext>,
    msgs_ctx: web::Data<MessagesContext>,
) -> ApiResult {
    let mut uploading = UploadingContext::default();

    while let Some(field) = payload
        .try_next()
        .await
        .map_err(|_| create_error!(source = MultipartParseError::FormReadingError))?
    {
        let Some(field_name) = field.name() else {
            continue;
        };

        match field_name {
            "media" => {
                if uploading.media.is_some() {
                    continue;
                }

                let content_type = field
                    .content_type()
                    .ok_or(create_error!(UnsupportedMediaType))?
                    .to_string();

                // We have to do this bad mapping because the
                // Field doesn't implement AsyncRead, which is required for storage.put
                let stream_reader =
                    StreamReader::new(field.map_err(|_| {
                        std::io::Error::other(MultipartParseError::FieldReadingError)
                    }));

                let put_result = data_ctx.store.put(stream_reader).await?;
                uploading.is_new_media = put_result.is_new;

                let media_id = MediaId(put_result.key.inner);
                let media = match put_result.is_new {
                    true => {
                        let media = Media {
                            id: media_id.clone(),
                            preview_key: media_id.into(),
                            created_at: Utc::now(),
                            blob_size: put_result.size as i64,
                            preview_size: 0,
                            content_type,
                            accent_color: None,
                            width: None,
                            height: None,
                            status: MediaStatus::Pending,
                        };
                        data_ctx.db.insert_media(&media).await?;
                        media
                    }
                    false => data_ctx
                        .db
                        .get_media(&media_id)
                        .await?
                        .ok_or(create_error!(MissingAssetMedia))?,
                };
                uploading.media = Some(media);
            }
            _ => {
                continue;
            }
        }
    }

    let Some(media) = uploading.media else {
        return Err(create_error!(MissingUploadMedia));
    };

    let asset = Asset {
        id: AssetId::new(),
        media_key: media.id.clone(),
        created_at: Utc::now(),
        title: uploading.title,
        caption: uploading.caption,
        source_url: uploading.source_url,
        deleted_at: None,
    };
    data_ctx.db.insert_asset(&asset).await?;

    if uploading.is_new_media {
        MediaWorkerTask::FinalizeUploadedMedia {
            id: media.id.clone(),
        }
        .send(&msgs_ctx.media_worker_sender)
        .await;
    }

    let response = AssetApi::from_domain(asset, media);
    Ok(HttpResponse::Created().json(response))
}
