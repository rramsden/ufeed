// Silice diesel warning about "Queryable" - Rust nightly issue
#![allow(proc_macro_derive_resolution_fallback)]

use rocket_contrib::{Json, Value};
use models::*;
use db::DbConn;
use auth::*;

#[get("/")]
pub fn index(connection: DbConn) -> Json<Value> {
  Json(json!(Post::read(&connection)))
}

#[post("/", data = "<post>")]
pub fn create(post: Json<NewPost>, connection: DbConn) -> Json<Post> {
  let insert = NewPost { ..post.into_inner() };
  Json(NewPost::create(insert, &connection))
}

#[get("/login")]
pub fn login(email: String, password: String) -> String {
  format!("Hello, {} year old named {}!", email, password)
}
