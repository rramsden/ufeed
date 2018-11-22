#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

// Macros!
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

extern crate r2d2;
extern crate r2d2_diesel;

// Module declarations!
mod auth;
mod models;
mod router;
mod routes;
mod db;
mod schema;

// Entry point!
fn main() {
  router::routes().launch();
}
