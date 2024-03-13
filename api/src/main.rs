#[macro_use]
extern crate rocket;

#[path = "routes/coffee.rs"]
mod coffee;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", coffee::routes())
}
