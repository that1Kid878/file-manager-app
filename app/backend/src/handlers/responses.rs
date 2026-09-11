use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

#[derive(Serialize)]
pub struct ModelSuccessResponse<T> {
    success: bool,
    data: T,
}

#[derive(Serialize)]
pub struct ModelErrorResponse {
    success: bool,
    message: String,
}

pub enum AppResponse<T: Serialize> {
    Ok(T),
    Created(T),
    NotFound(String),
    Conflict(String),
    BadRequest(String),
    Internal(String),
}

impl<T: Serialize> IntoResponse for AppResponse<T> {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Ok(data) => (
                StatusCode::OK,
                Json(ModelSuccessResponse {
                    success: true,
                    data,
                }),
            )
                .into_response(),
            Self::Created(data) => (
                StatusCode::CREATED,
                Json(ModelSuccessResponse {
                    success: true,
                    data,
                }),
            )
                .into_response(),
            Self::NotFound(message) => (
                StatusCode::NOT_FOUND,
                Json(ModelErrorResponse {
                    success: false,
                    message,
                }),
            )
                .into_response(),
            Self::Conflict(message) => (
                StatusCode::CONFLICT,
                Json(ModelErrorResponse {
                    success: false,
                    message,
                }),
            )
                .into_response(),
            Self::BadRequest(message) => (
                StatusCode::BAD_REQUEST,
                Json(ModelErrorResponse {
                    success: false,
                    message,
                }),
            )
                .into_response(),
            Self::Internal(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ModelErrorResponse {
                    success: false,
                    message,
                }),
            )
                .into_response(),
        }
    }
}
