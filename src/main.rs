#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod posts;
mod router;

fn main() {
    router::routes().launch();
}
