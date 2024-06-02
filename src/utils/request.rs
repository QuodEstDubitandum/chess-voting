use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MoveRequest {
    pub from: String,
    pub to: String,
    pub promotion: char,
}

#[derive(Deserialize, Debug)]
pub struct FinishRequest {
    pub game_id: String,
    pub game_result: String,
}
