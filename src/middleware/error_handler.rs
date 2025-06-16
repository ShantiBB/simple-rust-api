use actix_web::{
    error::{JsonPayloadError, PayloadError},
    http::StatusCode,
    HttpResponse, ResponseError,
};
use std::fmt;

#[derive(Debug)]
pub struct AppError {
    pub message: String,
    pub status_code: StatusCode,
}

impl AppError {
    #[allow(dead_code)]
    pub fn new(message: String, status_code: StatusCode) -> Self {
        Self {
            message,
            status_code,
        }
    }

    pub fn internal_server_error(message: String) -> Self {
        Self {
            message,
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn bad_request(message: String) -> Self {
        Self {
            message,
            status_code: StatusCode::BAD_REQUEST,
        }
    }

    #[allow(dead_code)]
    pub fn not_found(message: String) -> Self {
        Self {
            message,
            status_code: StatusCode::NOT_FOUND,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        self.status_code
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(serde_json::json!({
            "error": true,
            "message": self.message,
            "status_code": self.status_code().as_u16()
        }))
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        log::error!("Database error: {:?}", err);
        AppError::internal_server_error("Database error occurred".to_string())
    }
}

impl From<JsonPayloadError> for AppError {
    fn from(err: JsonPayloadError) -> Self {
        log::error!("JSON payload error: {:?}", err);
        AppError::bad_request("Invalid JSON payload".to_string())
    }
}

impl From<PayloadError> for AppError {
    fn from(err: PayloadError) -> Self {
        log::error!("Payload error: {:?}", err);
        AppError::bad_request("Invalid request payload".to_string())
    }
}
