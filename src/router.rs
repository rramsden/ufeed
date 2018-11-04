use posts::{controller};

pub fn routes() -> rocket::Rocket {
    rocket::ignite()
      .mount("/posts", routes![controller::index, controller::create])
}
