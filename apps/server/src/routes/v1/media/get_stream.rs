use actix_web::{HttpResponse, get, http::header, web};
use ms_content_store::key::StorageKey;
use ms_database::traits::MediaRepoExt;
use ms_shared_models::domains::MediaId;
use tokio_util::io::ReaderStream;

use crate::{
    di::DataContext,
    error::{AppError, AppResult},
};

#[get("/{id}")]
pub async fn get_media_stream(
    id: web::Path<MediaId>,
    ctx: web::Data<DataContext>,
) -> AppResult<HttpResponse> {
    let media = ctx.db.get_media(&id).await?.ok_or(AppError::NotFound)?;

    let content_type = media.content_type;
    let content_length = header::ContentLength(media.blob_size as usize);

    let storage_key = StorageKey::from_str_unchecked(&media.id.0);
    let reader = ctx.store.get(storage_key).await?;

    let stream = ReaderStream::new(reader);

    Ok(HttpResponse::Ok()
        .content_type(content_type)
        .append_header(content_length)
        .streaming(stream))
}
