use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TurnstileResponse {
    pub token: String,
    pub code: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SessionResponse {
    pub token: String,
    pub exp: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CobaltResponse {
    pub status: String,
    pub url: String,
    pub filename: String,
}
