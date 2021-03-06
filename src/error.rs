use rocket::{
    http::Status,
    request::Request,
    response::{Responder, Response},
};

/// This error implements `Responder`
/// so not all errors need to be handled
/// for it to be used.
#[derive(Debug, Error, Clone)]
#[non_exhaustive]
pub enum Error {
    /// Returns a `406 Not Acceptable` response.
    /// This error is thrown when none of the languages requested
    /// are supported.
    #[error("unsupported language.")]
    NotAcceptable,

    /// Used to respond with a 404 NotFound. This is used when dealing with
    /// unsupported language codes in the url.
    #[error("404 not found.")]
    NotFound,
}

impl Error {
    /// returns the http status for the error.
    pub fn status(&self) -> Status {
        match self {
            Self::NotAcceptable => Status::NotAcceptable,
            Self::NotFound => Status::NotFound,
        }
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'o> {
        Response::build()
            .status(self.status())
            .ok()
    }
}
