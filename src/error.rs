use failure::Fail;
use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Response, Result};
use rocket::Request;
use std::io::Cursor;

#[derive(Debug, Fail)]
pub enum CustomError {
    #[fail(display = "Database Error {}", 0)]
    DatabaseError(diesel::result::Error),
}

impl From<diesel::result::Error> for CustomError {
    fn from(err: diesel::result::Error) -> Self {
        CustomError::DatabaseError(err)
    }
}

impl<'r> Responder<'r, 'static> for CustomError {
    fn respond_to(self, _: &'r Request<'_>) -> Result<'static> {
        let body = format!("Diesel error: {}", self);
        let response = Response::build()
            .status(Status::InternalServerError)
            .header(ContentType::Plain)
            .sized_body(body.len(), Cursor::new(body))
            .finalize();

        Ok(response)
    }
}
