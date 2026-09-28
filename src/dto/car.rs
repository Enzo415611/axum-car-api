use serde::{Deserialize, Serialize};

use crate::db::model::CarState;

#[derive(Serialize, Deserialize)]
pub struct CreateCar {
    pub name: String,
    pub model: String,
    pub year: String,
    pub state: CarState,
}
