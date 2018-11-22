use routes;
use db;

pub fn routes() -> rocket::Rocket {
  rocket::ignite()
    .manage(db::init_pool())
    .mount("/posts", 
           routes![
           routes::index,
           routes::create,
           routes::login
           ])
}
