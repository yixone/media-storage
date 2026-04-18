use actix_web::{HttpResponse, get, http::header, web};
use tokio_util::io::ReaderStream;

use crate::{create_error, di::DataContext, error::Result, models::domains::MediaId};

/// Configures `/media` endpoints
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("media").service(get_media_stream));
}

/// Returns media stream by id
#[get("/{id}")]
pub async fn get_media_stream(
    path: web::Path<(MediaId,)>,
    ctx: web::Data<DataContext>,
) -> Result<HttpResponse> {
    let id = path.into_inner().0;
    let media = ctx
        .db
        .get_media(&id)
        .await?
        .ok_or(create_error!(NotFound))?;

    let content_type = media.mimetype;
    let content_length = header::ContentLength(media.size as usize);

    let storage_key = id.into();
    let reader = ctx.store.get(&storage_key).await?;
    let stream = ReaderStream::new(reader);

    Ok(HttpResponse::Ok()
        .content_type(content_type)
        .append_header(content_length)
        .streaming(stream))
}
