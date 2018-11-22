#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::pg::PgConnection;
use super::diesel::prelude::*;
use schema::posts;
use schema::posts::dsl::*;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Post {
  pub id: i32,
  pub title: String,
  pub body: String
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name="posts"]
pub struct NewPost {
  pub title: String,
  pub body: String,
}

impl Post {
  pub fn read(connection: &PgConnection) -> Vec<Post> {
    posts.load::<Post>(connection).unwrap()
  }
}

impl NewPost {
  pub fn create(post: NewPost, connection: &PgConnection) -> Post {
    diesel::insert_into(posts::table)
      .values(&post)
      .execute(connection)
      .expect("Error creating new post");

    posts::table.order(posts::id.desc()).first(connection).unwrap()
  }
}
