use actix_cors::Cors;
use actix_web::{App, HttpServer, web::Data};
use media_storage::{
    bg::media::MediaWorker,
    db::providers::Database,
    di::{DataContext, MsgsContext},
    error::Result,
    files::{host::FileHost, storage::Storage},
    routes,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initializing the logger
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .compact()
        .with_target(false)
        .init();

    // Open the database
    let db = Database::open_sqlite("data/data.db").await?;

    // Open storage
    let file_host = FileHost::fs("data/storage").await?;
    let store = Storage::new(file_host);

    // Launching background workers
    let media_worker = MediaWorker::new(db.clone(), store.clone());
    let media_worker_tx = media_worker.sender();
    media_worker.spawn().await;

    // Create an application context
    let ctx = Data::new(DataContext { db, store });
    let msg_ctx = Data::new(MsgsContext { media_worker_tx });

    // Create and launch the server
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .configure(routes::config)
            .app_data(ctx.clone())
            .app_data(msg_ctx.clone())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;
    Ok(())
}
