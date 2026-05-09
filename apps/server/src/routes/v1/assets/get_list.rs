use actix_web::{HttpResponse, get, web};

use crate::{di::DataContext, error::AppResult};

#[get("")]
pub async fn get_assets_list(ctx: web::Data<DataContext>) -> AppResult<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}
