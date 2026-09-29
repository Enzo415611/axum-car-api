use axum::Json;
use toasty::Db;

use crate::{
    db::model::Car,
    dto::car::{CarDto, CreateCarDto},
};

pub async fn save_car(db: &mut Db, car: CreateCarDto) -> Result<Json<CarDto>, Json<String>> {
    match toasty::create!(Car {
        name: car.name,
        model: car.model,
        year: car.year,
        state: car.state
    })
    .exec(db)
    .await
    {
        Ok(c) => Ok(Json(CarDto {
            id: c.id,
            model: c.model,
            name: c.name,
            year: c.year,
            state: c.state,
        })),
        Err(err) => Err(Json(err.to_string())),
    }
}

pub async fn get_all(db: &mut Db) -> Result<Json<Vec<CarDto>>, Json<String>> {
    match Car::all().exec(db).await {
        Ok(c) => {
            let v: Vec<CarDto> = c
                .into_iter()
                .map(|c| CarDto {
                    id: c.id,
                    model: c.model,
                    name: c.name,
                    state: c.state,
                    year: c.year,
                })
                .collect();
            Ok(Json(v))
        }
        Err(err) => Err(Json(err.to_string())),
    }
}

pub async fn update(
    id: u64,
    db: &mut Db,
    update_car: CreateCarDto,
) -> Result<Json<CarDto>, Json<String>> {
    let up = Car::update_by_id(id);
    let up = up
        .model(&update_car.model)
        .name(&update_car.name)
        .state(&update_car.state)
        .year(&update_car.year);

    if let Err(err) = up.exec(db).await {
        Err(Json(err.to_string()))
    } else {
        Ok(Json(CarDto {
            id,
            model: update_car.model,
            name: update_car.name,
            state: update_car.state,
            year: update_car.year,
        }))
    }
}
