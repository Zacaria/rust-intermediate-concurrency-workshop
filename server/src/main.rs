use anyhow::Result;

use crate::{
    router::get_router,
    sqlx_client::{get_connection_pool, run_migrations},
};

mod blog_client;
// mod save_domain;
mod router;
mod sqlx_client;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").unwrap();
    println!("DB url : {database_url:?}");
    let pool = get_connection_pool(&database_url).await?;

    println!("Running migrations");
    run_migrations(pool.clone()).await?;

    let listen_address = std::env::var("LISTEN_ADDRESS").unwrap();

    let listener = tokio::net::TcpListener::bind(&listen_address).await?;

    let app = get_router(pool.clone());

    println!("Listening on: {listen_address}");
    axum::serve(listener, app).await?;

    Ok(())
}
