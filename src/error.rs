use rocket_contrib::json::Json;
use rocket::http::Status;
use std::default::Default;
use rocket::Request;
use rocket::response::Responder;
use rocket::response;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMessage {
    pub status_code: u16,
    pub message: &'static str,
}

impl<'r> Responder<'r> for ErrorMessage {
    fn respond_to(self, request: &Request) -> response::Result<'r> {
        let status_code = self.status_code;
        Json(self)
            .respond_to(&request)
            .map(|mut s| {
                s.set_status(Status::raw(status_code));
                s
            })
    }
}

impl Default for ErrorMessage {
    fn default() -> Self {
        ErrorMessage { 
            status_code: 500,
            message: "Something went wrong"
        }
    }
}
