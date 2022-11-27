use actix_web::{error, http::StatusCode, HttpResponse};
use log::debug;
use sea_orm::DbErr;
use derive_more::{Display, Error};
use serde::Serialize;

#[derive(Debug, Display, Error, Serialize)]
pub struct ErrorResponder {
    message: String,
}

impl error::ResponseError for ErrorResponder {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().json(self)
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

// The following impl's are for easy conversion of error types.

impl From<DbErr> for ErrorResponder {
    fn from(err: DbErr) -> ErrorResponder {
        debug!("{:?}", &err);

        ErrorResponder {
            message: err.to_string(),
        }
    }
}

impl From<String> for ErrorResponder {
    fn from(string: String) -> ErrorResponder {
        ErrorResponder { message: string }
    }
}

impl From<&str> for ErrorResponder {
    fn from(str: &str) -> ErrorResponder {
        str.to_owned().into()
    }
}
