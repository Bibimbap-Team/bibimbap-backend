#[macro_use]
extern crate rocket;

use bibimbap_backend::startup::run;
use rocket::http::Status;
use rocket::local::blocking::Client;
use rocket::{Build, Rocket};
use std::io::Read;

fn spawn_app() -> Rocket<Build> {
    run()
}

#[test]
fn health_check_works() {
    let client = Client::tracked(spawn_app()).expect("Failed to create client");
    let response = client.get(uri!("/health_check")).dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.bytes().count(), 0);
}
