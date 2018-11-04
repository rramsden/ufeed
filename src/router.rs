use posts;

pub fn routes() -> rocket::Rocket {
  rocket::ignite()
    .mount("/posts", 
           routes![
           posts::index,
           posts::create,
           posts::update,
           posts::delete
           ])
}
