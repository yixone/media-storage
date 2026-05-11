use actix_web::{HttpResponse, get, web};
use serde::Deserialize;
use shelf_content_store::key::StorageKey;
use shelf_database::traits::MediaRepoExt;
use shelf_shared_models::domains::{MediaError, MediaId};
use tokio_util::io::ReaderStream;

use crate::{di::DataContext, error::AppResult};

#[derive(Deserialize, Default)]
#[serde(rename_all = "lowercase")]
enum MediaReturnFormat {
    #[default]
    Original,
    Preview,
}

#[derive(Deserialize)]
struct GetMediaStreamQuery {
    #[serde(default)]
    format: MediaReturnFormat,
}

#[get("/{id}")]
pub async fn get_media_stream(
    id: web::Path<MediaId>,
    query: web::Query<GetMediaStreamQuery>,
    ctx: web::Data<DataContext>,
) -> AppResult<HttpResponse> {
    let query = query.into_inner();
    let format = query.format;

    let media = ctx
        .db
        .get_media(&id)
        .await?
        .ok_or(MediaError::MediaNotFound)?;

    let content_type = match format {
        MediaReturnFormat::Original => media.content_type,
        MediaReturnFormat::Preview => "image/webp".into(),
    };

    let storage_key = StorageKey::from_str_unchecked(match format {
        MediaReturnFormat::Original => &media.id.0,
        MediaReturnFormat::Preview => &media.thumbnail_key.0,
    });

    let reader = ctx.store.get(storage_key).await?;

    let stream = ReaderStream::new(reader);

    Ok(HttpResponse::Ok()
        .content_type(content_type)
        .streaming(stream))
}
