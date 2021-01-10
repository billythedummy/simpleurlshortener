use actix_web::body::Body;
use actix_web::http::StatusCode;
use actix_web::web::HttpResponse;
use actix_web::ResponseError;
use std::fmt;

#[derive(Debug)]
pub struct Error {
    pub pub_reason: String,
    pub status_code: u16,
}

macro_rules! ImplDbIntoError {
    ($err: ty) => {
        impl From<$err> for Error {
            fn from(e: $err) -> Self {
                eprintln!("postgres error: {}", e);
                Self {
                    pub_reason: "Database error".into(),
                    status_code: 500,
                }
            }
        }
    };
}

ImplDbIntoError! {tokio_postgres::error::Error}
ImplDbIntoError! {deadpool_postgres::PoolError}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.pub_reason, self.status_code)
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match StatusCode::from_u16(self.status_code) {
            Ok(sc) => sc,
            Err(e) => {
                eprintln!("Invalid status code: {}, err: {}", self.status_code, e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    fn error_response(&self) -> HttpResponse<Body> {
        self.pub_reason.clone().into()
    }
}
