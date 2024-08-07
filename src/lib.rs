#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

#[get("/health_check")]
fn health_check() {
    ()
}

pub fn run() -> Rocket<Build> {
    rocket::build().mount("/", routes![health_check])
}
