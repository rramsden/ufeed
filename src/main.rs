#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

// Macros!
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

// Module declarations!
mod posts;
mod router;

// Entry point!
fn main() {
    router::routes().launch();
}
