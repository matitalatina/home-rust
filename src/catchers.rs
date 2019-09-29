use crate::error::ErrorMessage;
use rocket_contrib::json::Json;

#[catch(500)]
pub fn internal_server_error() -> Json<ErrorMessage> {
    Json(ErrorMessage {
        message: "Something went wrong",
        ..ErrorMessage::default()
    })
}

#[catch(404)]
pub fn not_found_error() -> Json<ErrorMessage> {
    Json(ErrorMessage {
        message: "API Not found",
        status_code: 404,
        ..ErrorMessage::default()
    })
}
