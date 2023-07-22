use axum::{http::StatusCode, response::IntoResponse};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, strum_macros::AsRefStr)]
pub enum Error {
    LoginFail,
    AuthFaileNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    TicketDeleteFailIdNotFound { id: u64 },
    AuthFailCtxNotInRequestExtension,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        // Create a placeholder Axum response.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Error::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            Error::AuthFaileNoAuthTokenCookie |
            Error::AuthFailTokenWrongFormat |
            Error::AuthFailCtxNotInRequestExtension => {
                (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
            }
            Error::TicketDeleteFailIdNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVICE_ERROR)
        }
    }
}
#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}