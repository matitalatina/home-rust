use super::catchers;
use super::conditioner;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn start() {
    rocket::ignite()
        .mount("/", routes![index, conditioner::controller::activate])
        .register(catchers![
            catchers::internal_server_error,
            catchers::not_found_error,
        ])
        .launch();
}
