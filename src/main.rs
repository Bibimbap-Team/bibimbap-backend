#[macro_use]
extern crate rocket;

#[get("/health_check")]
fn health_check() {
    ()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health_check])
}
