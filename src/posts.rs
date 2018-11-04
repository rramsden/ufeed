mod posts {
  #[derive(Serialize, Deserialize)]
  pub struct Post {
    pub id: Option<i32>,
    pub title: String,
    pub body: String
  }

  pub mod controller {
    use rocket_contrib::{Json, Value};

    #[get("/")]
    pub fn index() -> Json<Value> {
      Json(json!([
                 "post 1",
                 "post 2"
      ]))
    }

    #[post("/", data = "<post>")]
    pub fn create(post: Json<super::Post>) -> Json<super::Post> {
      post
    }
  }
}
