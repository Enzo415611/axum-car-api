use serde::{Deserialize, Serialize};
// bank model
#[derive(Clone, Debug, toasty::Model)]
pub struct Car {
    #[key]
    #[auto]
    pub id: u64,

    pub name: String,
    pub model: String,
    pub year: String,
    pub state: CarState,
}

impl Default for Car {
    fn default() -> Self {
        Self {
            id: 0,
            model: "".to_string(),
            name: "".to_string(),
            year: "".to_string(),
            state: CarState::New,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, toasty::Embed)]
pub enum CarState {
    New,
    Used,
}
