#[macro_use]
extern crate rocket;

use rocket::fs::NamedFile;
use rocket::response::status::NotFound;
use std::path::PathBuf;

#[get("/")]
async fn index() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("static/index.html")
        .await
        .map_err(|e| NotFound(e.to_string()))
}

#[get("/<path..>")]
async fn static_path(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = PathBuf::from("static").join(path);

    NamedFile::open(path)
        .await
        .map_err(|e| NotFound(e.to_string()))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, static_path])
}
