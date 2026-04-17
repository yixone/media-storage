use actix_web::{App, HttpServer, web::Data};
use media_storage::{
    db::providers::Database,
    di::DataContext,
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

    // Create an application context
    let ctx = Data::new(DataContext { db, store });

    // Create and launch the server
    HttpServer::new(move || App::new().configure(routes::config).app_data(ctx.clone()))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}
