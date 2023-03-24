use serde::{Deserialize, Serialize};
use worker::{Cors, Headers, Response};

fn build_headers() -> worker::Result<Headers> {
    let mut headers = Headers::new();
    headers.set("Content-Type", "application/json")?;
    Ok(headers)
}

pub fn build_response(
    logger_response: LoggerResponse,
    status_code: u16,
) -> worker::Result<Response> {
    Response::ok(serde_json::to_string(&logger_response).unwrap())?
        .with_headers(build_headers()?)
        .with_status(status_code)
        .with_cors(&Cors::new().with_origins(vec!["*"]))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoggerResponse {
    #[serde(rename = "response")]
    pub response_type: ResponseType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ResponseType {
    #[serde(rename = "BAD_REQUEST")]
    BadRequest,
    #[serde(rename = "OK")]
    Ok,
    #[serde(rename = "IPLOGGER_DETECTED")]
    IpLoggerDetected,
}
