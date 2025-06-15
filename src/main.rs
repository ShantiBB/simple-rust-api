mod config;
mod repository;
mod models;
mod handler;

use std::fs;
use anyhow::{anyhow, Result };
use axum::{ Router, routing::{ get, put } };
use tokio::net::TcpListener;
use tower_http::timeout::TimeoutLayer;

use crate::config::config::Config;
use crate::handler::item::{
    create_item,
    delete_item_by_id,
    get_all_items,
    get_item_by_id,
    update_item_by_id
};
use crate::repository::postgres::PostgresRepository;

#[tokio::main]
async fn main() -> Result<()> {
    let config_str = fs::read_to_string("src/config/local.yml")
        .map_err(|e| anyhow!("Filed to read config {}", e))?;
    let config: Config = serde_yaml::from_str(&config_str)
        .map_err(|e| anyhow!("Filed to parse config {}", e))?;

    let db_url = &config.database.url;
    let pg_repo = PostgresRepository::new(db_url).await?;
    let router = Router::new()
        .route("/items", get(get_all_items).post(create_item))
        .route("/items/{id}", get(get_item_by_id))
        .route("/items/{id}", put(update_item_by_id).delete(delete_item_by_id))
        .with_state(pg_repo)
        .layer(TimeoutLayer::new(std::time::Duration::new(5, 0)));

    let addr = &config.web.addr;
    let listener = TcpListener::bind(addr)
        .await
        .map_err(|e| anyhow!("Error listen server {}", e))?;

    axum::serve(listener, router).await
        .map_err(|e| anyhow!("Error start server {}", e))?;

    Ok(())
}
