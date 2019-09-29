#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket;

pub mod catchers;
pub mod conditioner;
pub mod error;
pub mod server;

fn main() {
    server::start()
}
