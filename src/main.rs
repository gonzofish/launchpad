#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;

mod api;
mod error;
mod models;
mod schema;
mod static_routes;
#[cfg(test)]
mod tests;

use diesel::PgConnection;
use dotenv::dotenv;
use rocket::figment::map;
use rocket::figment::value::{Map, Value};
use std::env;

#[database("launchpad_db")]
struct DBPool(PgConnection);

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap();
    let db: Map<_, Value> = map! {
        "pool_size" => 10.into(),
        "url" => db_url.into(),
    };
    let figment = rocket::Config::figment().merge(("databases", map!["launchpad_db" => db]));

    rocket::custom(figment)
        .mount("/", static_routes::get_routes())
        .mount("/api", api::routes::get_routes())
        .attach(DBPool::fairing())
}
