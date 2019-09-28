use super::conditioner;
use super::catchers;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn start() {
    rocket::ignite()
        .mount("/", routes![index, conditioner::controller::activate])
        .register(catchers![catchers::internal_server_error])
        .launch();
}