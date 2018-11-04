#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod posts;

#[get("/")]
fn index() -> String {
  "Hello World".to_string()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
      .mount("/", routes![index])
      .mount("/posts", vec![posts::controller::index, posts::controller::create])
}

fn main() {
    rocket().launch();
}
