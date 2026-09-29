use serde::{Deserialize, Serialize};

use crate::db::model::CarState;

#[derive(Deserialize)]
pub struct CreateCarDto {
    pub name: String,
    pub model: String,
    pub year: String,
    pub state: CarState,
}

#[derive(Serialize)]
pub struct CarDto {
    pub id: u64,
    pub name: String,
    pub model: String,
    pub year: String,
    pub state: CarState,
}
