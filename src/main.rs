use bibimbap_backend::run;

#[rocket::main]
async fn main() {
    if let Err(e) = run().launch().await {
        println!("Failed to launch Rocket: {e}");
        drop(e);
    }
}
