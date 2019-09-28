#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket;

pub mod conditioner;
pub mod server;
pub mod error;
pub mod catchers;

fn main() {
    server::start();
}