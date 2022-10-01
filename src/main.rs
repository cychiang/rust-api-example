#[macro_use] extern crate rocket;

fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() {
    let _ = rocket::build().mount("/", routes![index]).launch().await.expect("TODO: panic message");
}
