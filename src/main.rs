use std::sync::Arc;

use quests_tracker::{
    config::loader,
    infrastructure::{axum::http_serve::start, postgresql::connection},
};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let dotenv = match loader::load() {
        Ok(env) => env,
        Err(e) => {
            error!("Failed to load ENV: {}", e);
            std::process::exit(1);
        }
    };

    info!("ENV has been loaded");

    let postgres_pool = match connection::establish_connection(&dotenv.database.url) {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to establish PostgreSQL connection: {}", e);
            std::process::exit(1);
        }
    };

    info!("PostgreSQL connection established");

    start(Arc::new(dotenv), Arc::new(postgres_pool))
        .await
        .expect("Failed to start server.");
}
