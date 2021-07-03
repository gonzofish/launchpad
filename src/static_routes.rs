use rocket::fs::NamedFile;
use rocket::response::status::NotFound;
use rocket::Route;

use std::path::PathBuf;

pub fn get_routes() -> Vec<Route> {
    routes![index, static_path]
}

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
