use crate::routes::health_check;
use rocket::{routes, Build, Rocket};

pub fn run() -> Rocket<Build> {
    rocket::build().mount("/", routes![health_check::health_check])
}
