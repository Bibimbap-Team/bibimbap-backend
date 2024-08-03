#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::local::blocking::Client;
use rocket::{Build, Rocket};
use std::io::Read;

fn spawn_app() -> Rocket<Build> {
    bibimbap_backend::run()
}

#[test]
fn health_check_works() {
    let client = Client::tracked(spawn_app()).expect("Failed to create client");
    let response = client.get(uri!("/health_check")).dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.bytes().count(), 0);
}
