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

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use std::io::Read;

    #[test]
    fn test_hello() {
        let client = Client::tracked(rocket()).expect("Failed to create client");
        let response = client.get(uri!(super::health_check)).dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.bytes().count(), 0);
    }
}
