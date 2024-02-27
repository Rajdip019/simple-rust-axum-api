use axum::{http::StatusCode, response::{Response, IntoResponse}};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,

    // Ai=uth Errors
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,

    // -- Model errors
    TicketDeleteFailIdNotFound { id : u64 },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("{:?}", self);
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
    }
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
// end region: --- Error Boilerplate