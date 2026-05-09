use actix_multipart::Multipart;
use actix_web::{HttpResponse, post, web};
use chrono::Utc;
use futures::TryStreamExt;
use ms_database::traits::{AssetRepoExt, MediaRepoExt};
use ms_shared_models::domains::{
    Asset, AssetError, AssetId, Media, MediaError, MediaId, MediaStatus,
};

use crate::{
    bg::media::MediaWorkerTask,
    di::{DataContext, MessagesContext},
    error::{AppError, AppResult},
    http::multipart::read_field_to_string,
    models::v1::asset::ApiAsset,
};

#[derive(Default)]
struct UploadingContext {
    uploaded_media: Option<Media>,
    is_new_media: bool,

    title: Option<String>,
    caption: Option<String>,

    source_url: Option<String>,
}

#[post("")]
pub async fn upload_asset(
    mut payload: Multipart,
    ctx: web::Data<DataContext>,
    msgs_ctx: web::Data<MessagesContext>,
) -> AppResult<HttpResponse> {
    let mut uploading = UploadingContext::default();

    while let Some(mut field) = payload.try_next().await? {
        let Some(field_name) = field.name() else {
            continue;
        };

        match field_name {
            "file" => {
                if uploading.uploaded_media.is_some() {
                    continue;
                }

                let mimtype = field
                    .content_type()
                    .ok_or(MediaError::InvalidMimetype)?
                    .to_string();

                // FIXME: fix this ugly errors mapping
                let put_res = ctx
                    .store
                    .put_stream(field.map_err(AppError::from).map_err(std::io::Error::other))
                    .await?;
                uploading.is_new_media = put_res.is_new;

                let media_id = MediaId(put_res.key.inner);
                let media = match put_res.is_new {
                    true => {
                        let media = Media {
                            id: media_id,
                            created_at: Utc::now(),
                            blob_size: put_res.size as i64,
                            content_type: mimtype,
                            color: None,
                            width: None,
                            height: None,
                            status: MediaStatus::Pending,
                        };
                        ctx.db.insert_media(&media).await?;
                        media
                    }
                    false => ctx
                        .db
                        .get_media(&media_id)
                        .await?
                        .ok_or(MediaError::MediaNotFound)?,
                };

                uploading.uploaded_media = Some(media);
            }
            "title" => uploading.title = Some(read_field_to_string(&mut field).await?),
            "caption" => uploading.caption = Some(read_field_to_string(&mut field).await?),
            "source_url" => uploading.source_url = Some(read_field_to_string(&mut field).await?),
            _ => {
                continue;
            }
        }
    }

    let Some(media) = uploading.uploaded_media else {
        return Err(AssetError::MissingUploadMedia.into());
    };

    let asset = Asset {
        id: AssetId::new(),
        media: media.id.clone(),
        created_at: Utc::now(),
        title: uploading.title,
        caption: uploading.caption,
        source_url: uploading.source_url,
    };
    ctx.db.insert_asset(&asset).await?;

    if uploading.is_new_media {
        MediaWorkerTask::NewMedia {
            id: media.id.clone(),
        }
        .send(&msgs_ctx.media_worker_msgs)
        .await;
    }

    let api_asset = ApiAsset::from_domains(asset, media);
    Ok(HttpResponse::Created().json(api_asset))
}
