use actix_web::{HttpResponse, get, web};
use chrono::Utc;

use crate::{SERVER_VERSION, di::ServerContext, models::ServerInfoResponse};

/// Returns information about the server
#[get("/server")]
pub async fn about_server(server_ctx: web::Data<ServerContext>) -> HttpResponse {
    let data = ServerInfoResponse {
        version: SERVER_VERSION,
        runtime: Utc::now().timestamp() - server_ctx.runned_at.timestamp(),
    };

    HttpResponse::Ok().json(data)
}
