#![allow(unused)]
use rocket::http::{ContentType, Status};
use rocket::response;
use rocket::serde::json::Json;
use rocket::{Request, Response};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
pub struct BotResponse {
    pub success: bool,
    pub message: String
}

#[derive(Deserialize)]
pub struct JsonResponse {
    pub r#type: String, // Use `r#type` because `type` is a reserved keyword in Rust
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub chat: String
}

/// The struct we return for success responses (200s)
#[derive(Debug)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub json: Option<Json<T>>,
    pub status: Status,
}

/// Implements the `Responder` trait for Rocket, so we can simply return a for
/// endpoint functions, result and Rocket takes care of the rest.
impl<'r, T: Serialize> response::Responder<'r, 'r> for ApiResponse<T> {
    fn respond_to(self, req: &'r Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(req)?)
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}