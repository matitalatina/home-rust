use crate::error::ErrorMessage;
use std::io;
use rocket_contrib::json::Json;
use std::process::Command;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
  message: String,
}

#[post("/conditioner", format = "json")]
pub fn activate() -> Result<Json<Message>, ErrorMessage> {
  match Command::new("say").arg("Attiva condizionatore").spawn() {
    io::Result::Ok(_) => Ok(Json(Message {
      message: "ok!".to_string(),
    })),
    io::Result::Err(_) => Err(ErrorMessage{
      message: "I can't start process",
      ..ErrorMessage::default()
    })
  }
}
