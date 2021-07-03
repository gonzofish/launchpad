use super::rocket;
use rocket::http::Status;
use rocket::local::blocking::Client;

use std::fs;

#[test]
fn static_index() -> Result<(), String> {
    let client = Client::tracked(rocket()).expect("Rocket instance is invalid");
    let response = client.get("/").dispatch();
    let index_contents = get_static_file("index.html")?;

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string(), Some(index_contents));

    Ok(())
}

#[test]
fn static_404() {
    let client = Client::tracked(rocket()).expect("Rocket instance is invalid");
    let response = client.get("/unknown_route").dispatch();

    assert_eq!(response.status(), Status::NotFound);
}

fn get_static_file(file: &str) -> Result<String, String> {
    let filename = format!("./static/{}", file);
    let contents = fs::read_to_string(filename).expect("Couldn't open file!");

    Ok(contents)
}
