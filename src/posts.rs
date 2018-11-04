use rocket_contrib::{Json, Value};

#[derive(Serialize, Deserialize)]
pub struct Post {
  pub id: Option<i32>,
  pub title: String,
  pub body: String
}

#[get("/")]
pub fn index() -> Json<Value> {
  Json(json!([
             "post 1",
             "post 2"
  ]))
}

#[put("/<id>", data = "<post>")]
pub fn update(id: i32, post: Json<Post>) -> Json<Post> {
  post
}

#[delete("/<id>")]
pub fn delete(id: i32) -> Json<Value> {
  Json(json!({"status": "ok"}))
}

#[post("/", data = "<post>")]
pub fn create(post: Json<Post>) -> Json<Post> {
  post
}
