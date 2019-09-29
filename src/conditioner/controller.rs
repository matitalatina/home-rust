use crate::error::ErrorMessage;
use rocket_contrib::json::Json;
use std::io;
use std::process::Command;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateACResponse {
  message: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateACRequest {
  activate: bool,
}

#[put("/conditioner", format = "json", data = "<request>")]
pub fn activate(request: Json<UpdateACRequest>) -> Result<Json<UpdateACResponse>, ErrorMessage> {
  let command = if request.activate {
    activate_command
  } else {
    deactivate_command
  };
  match command() {
    io::Result::Ok(_) => Ok(Json(UpdateACResponse {
      message: "ok!".to_string(),
    })),
    io::Result::Err(_) => Err(ErrorMessage {
      message: "I can't start process",
      ..ErrorMessage::default()
    }),
  }
}

fn activate_command() -> Result<std::process::Child, io::Error> {
  Command::new("say").arg("Attiva condizionatore").spawn()
}

fn deactivate_command() -> Result<std::process::Child, io::Error> {
  Command::new("say").arg("Disattiva condizionatore").spawn()
}
