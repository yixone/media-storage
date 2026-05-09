use actix_web::{HttpResponse, post, web};

use crate::{di::DataContext, error::AppResult};

#[post("")]
pub async fn upload_asset(ctx: web::Data<DataContext>) -> AppResult<HttpResponse> {
    Ok(HttpResponse::Created().finish())
}
