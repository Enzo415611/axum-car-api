mod db;
mod dto;

use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};

use crate::db::db::init_db;
use crate::db::model::Car;
use crate::dto::car::CreateCar;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = init_db().await?;
    let app = Router::new()
        .route("/save", post(save_car))
        .route("/get", get(get_all_cars))
        .route("/delete", delete(delete_car))
        .route("/update", put(update_car))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn save_car(
    State(mut db): State<toasty::Db>,
    Json(car): Json<CreateCar>,
) -> (StatusCode, Result<Json<CreateCar>, Json<String>>) {
    match toasty::create!(Car {
        name: car.name,
        model: car.model,
        year: car.year,
        state: car.state
    })
    .exec(&mut db)
    .await
    {
        Ok(c) => (
            StatusCode::CREATED,
            Ok(Json(CreateCar {
                model: c.model,
                name: c.name,
                year: c.year,
                state: c.state,
            })),
        ),
        Err(err) => {
            println!("{}", err);
            (
                StatusCode::NOT_FOUND,
                Err(Json(format!("Err: {}", err.to_string()))),
            )
        }
    }
}

async fn get_all_cars(
    State(mut db): State<toasty::Db>,
) -> (StatusCode, Result<Json<Vec<CreateCar>>, Json<String>>) {
    match Car::all().exec(&mut db).await {
        Ok(c) => {
            let v: Vec<CreateCar> = c
                .into_iter()
                .map(|c| CreateCar {
                    model: c.model,
                    name: c.name,
                    state: c.state,
                    year: c.year,
                })
                .collect();
            (StatusCode::OK, Ok(Json(v)))
        }
        Err(err) => (StatusCode::NOT_FOUND, Err(Json(err.to_string()))),
    }
}

async fn delete_car() {}
async fn update_car() {}
