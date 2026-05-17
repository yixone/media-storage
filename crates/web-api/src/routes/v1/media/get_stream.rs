use actix_web::{
    HttpResponse, get,
    http::header::{self, ContentLength},
    web,
};
use asset_shelf_database::traits::MediaRepositoryExt;
use asset_shelf_models::domains::MediaId;
use asset_shelf_result::create_error;
use asset_shelf_storage::StorageKey;
use serde::Deserialize;
use tokio_util::io::ReaderStream;

use crate::{di::DataContext, routes::ApiResult};

/// Return media format
#[derive(Deserialize, Default)]
#[serde(rename_all = "lowercase")]
enum MediaReturnFormat {
    #[default]
    Original,
    Preview,
}

/// Query parameters for returning media
#[derive(Deserialize)]
struct GetMediaStreamQuery {
    #[serde(default)]
    format: MediaReturnFormat,
}

#[get("/{id}")]
pub async fn get_media_stream(
    id: web::Path<MediaId>,
    query: web::Query<GetMediaStreamQuery>,
    data_ctx: web::Data<DataContext>,
) -> ApiResult {
    let query = query.into_inner();
    let format = query.format;

    let media = data_ctx
        .db
        .get_media(&id)
        .await?
        .ok_or(create_error!(NotFound))?;

    let (content_type, content_length) = match format {
        MediaReturnFormat::Original => (media.content_type, media.blob_size),
        MediaReturnFormat::Preview => ("image/webp".into(), media.preview_size),
    };

    let storage_key = StorageKey::from_str_unchecked(match format {
        MediaReturnFormat::Original => &media.id.0,
        MediaReturnFormat::Preview => &media.preview_key.0,
    });

    let reader = data_ctx.store.get(storage_key).await?;
    let stream = ReaderStream::new(reader);

    Ok(HttpResponse::Ok()
        .content_type(content_type)
        .append_header(ContentLength(content_length as usize))
        .streaming(stream))
}
