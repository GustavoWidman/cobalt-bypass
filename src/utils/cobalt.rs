use log::{debug, error};

use crate::utils::{
    error::CobaltStage,
    types::{CobaltResponse, SessionResponse, TurnstileResponse},
};

use super::error::CobaltError;

pub async fn cobalt(url: String) -> Result<CobaltResponse, CobaltError> {
    let client = reqwest::Client::new();

    let turnstile_solver_host = dotenv!("TURNSTILE_SOLVER_HOST");
    let turnstile_solver_port = dotenv!("TURNSTILE_SOLVER_PORT");
    let turnstile_response = client
        .post(format!(
            "http://{}:{}/cf-clearance-scraper",
            turnstile_solver_host, turnstile_solver_port
        ))
        .json(&serde_json::json!({
            "url": "https://challenges.cobalt.tools/",
            "siteKey": &crate::consts::TURNSTILE_KEY,
            "mode": "turnstile-min",
        }))
        .send()
        .await
        .map_err(|e| {
            error!("Turnstile request error: {}", e);

            CobaltError {
                message: format!("Failed to fetch turnstile response: {}", e),
                stage: CobaltStage::Turnstile,
            }
        })?;

    debug!("Turnstile response: {:?}", turnstile_response);

    let turnstile_response = turnstile_response
        .error_for_status()
        .map_err(|e| {
            error!("Turnstile request error: {}", e);

            CobaltError {
                message: "Could not solve turnstile challenge, please try again later.".to_string(),
                stage: CobaltStage::Final,
            }
        })?
        .json::<TurnstileResponse>()
        .await
        .map_err(|e| {
            error!("Turnstile response parsing error: {}", e);

            CobaltError {
                message: format!("Failed to parse turnstile response: {}", e),
                stage: CobaltStage::Turnstile,
            }
        })?;

    debug!("Turnstile parsed: {:?}", turnstile_response);

    let token_response = client
        .post("https://api.cobalt.tools/session")
        .header("cf-turnstile-response", turnstile_response.token)
        .send()
        .await
        .map_err(|e| {
            error!("Session request error: {}", e);

            CobaltError {
                message: format!("Failed to fetch session response: {}", e),
                stage: CobaltStage::Session,
            }
        })?;

    debug!("Session response: {:?}", token_response);

    let token_response = token_response
        .error_for_status()
        .map_err(|e| {
            error!("Session request error: {}", e);

            CobaltError {
                message: "Cobalt session API returned an error, please try again later".to_string(),
                stage: CobaltStage::Final,
            }
        })?
        .json::<SessionResponse>()
        .await
        .map_err(|e| {
            error!("Session response parsing error: {}", e);
            CobaltError {
                message: format!("Failed to parse session response: {}", e),
                stage: CobaltStage::Session,
            }
        })?;

    debug!("Session parsed: {:?}", token_response);

    let cobalt_response = client
        .post("https://api.cobalt.tools/")
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .bearer_auth(token_response.token)
        .json(&serde_json::json!({
            "url": url,
        }))
        .send()
        .await
        .map_err(|e| {
            error!("Cobalt request error: {}", e);

            CobaltError {
                message: format!("Failed to fetch cobalt response: {}", e),
                stage: CobaltStage::Final,
            }
        })?;

    debug!("Cobalt response: {:?}", cobalt_response);

    let cobalt_response = cobalt_response
        .error_for_status()
        .map_err(|e| {
            error!("Cobalt response error: {}", e);

            CobaltError {
                message: "Cobalt API returned an error, please use another URL or try again later"
                    .to_string(),
                stage: CobaltStage::Final,
            }
        })?
        .json::<CobaltResponse>()
        .await
        .map_err(|e| {
            error!("Cobalt response parsing error: {}", e);

            CobaltError {
                message: format!("Failed to parse cobalt response: {}", e),
                stage: CobaltStage::Final,
            }
        })?;

    debug!("Cobalt parsed: {:?}", cobalt_response);

    Ok(cobalt_response)
}
