use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum CobaltStage {
    #[serde(rename = "turnstile")]
    Turnstile,

    #[serde(rename = "session")]
    Session,

    #[serde(rename = "final")]
    Final,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CobaltError {
    pub message: String,
    pub stage: CobaltStage,
}
