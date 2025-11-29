use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
#[error("The requested resource was not found")]
pub struct NotFoundError;

pub struct AppError(anyhow::Error);
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        error!("Internal server error: {:#}", self.0);

        // try to downcast the error to check for specific types
        if self.0.is::<NotFoundError>() {
            return (StatusCode::NOT_FOUND, "Not Found".to_string()).into_response();
        }
        if let Some(sqlx_err) = self.0.downcast_ref::<sqlx::Error>() {
            if let sqlx::Error::RowNotFound = sqlx_err {
                return (StatusCode::NOT_FOUND, "Not Found".to_string()).into_response();
            }
        }

        // for any other error, return 500
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong".to_string(),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
