use rocket_contrib::json::Json;
use crate::error::ErrorMessage;

#[catch(500)]
pub fn internal_server_error() -> Json<ErrorMessage> {
    Json(ErrorMessage {
        message: "Something went wrong",
        ..ErrorMessage::default()
    })
}
