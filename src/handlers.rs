use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    dto::car::{CarDto, CreateCarDto},
    services::{get_all, save_car, update},
};

pub async fn create_car(
    State(mut db): State<toasty::Db>,
    Json(car): Json<CreateCarDto>,
) -> (StatusCode, Result<Json<CarDto>, Json<String>>) {
    match save_car(&mut db, car).await {
        Ok(c) => (StatusCode::CREATED, Ok(c)),
        Err(err) => (StatusCode::NOT_FOUND, Err(err)),
    }
}

pub async fn get_all_cars(
    State(mut db): State<toasty::Db>,
) -> (StatusCode, Result<Json<Vec<CarDto>>, Json<String>>) {
    match get_all(&mut db).await {
        Ok(v) => (StatusCode::OK, Ok(v)),
        Err(err) => (StatusCode::NOT_FOUND, Err(err)),
    }
}
pub async fn update_car(
    Path(id): Path<u64>,
    State(mut db): State<toasty::Db>,
    Json(update_car): Json<CreateCarDto>,
) -> (StatusCode, Result<Json<CarDto>, Json<String>>) {
    match update(id, &mut db, update_car).await {
        Ok(c) => (StatusCode::OK, Ok(c)),
        Err(err) => (StatusCode::NOT_MODIFIED, Err(err)),
    }
}

pub async fn delete_car() {}
