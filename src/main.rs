mod db;
mod dto;
mod handlers;
mod services;

use axum::Router;
use axum::routing::{delete, get, post, put};

use crate::db::db::init_db;
use crate::handlers::{create_car, delete_car, get_all_cars, update_car};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = init_db().await?;
    let app = Router::new()
        .route("/save", post(create_car))
        .route("/get", get(get_all_cars))
        .route("/update/{id}", put(update_car))
        .route("/delete", delete(delete_car))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
