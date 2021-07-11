#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;

mod api;
pub mod error;
pub mod models;
mod schema;
mod static_routes;

use std::env;

use diesel::PgConnection;
use dotenv::dotenv;
use rocket::figment::value::{Map, Value};
use rocket::figment::{map, Figment};
use rocket::{Build, Rocket};

#[database("launchpad_db")]
pub struct DBPool(PgConnection);

pub fn blast_off(db_url: &str) -> Rocket<Build> {
    dotenv().ok();

    let figment = setup_db(db_url);

    rocket::custom(figment)
        .mount("/", static_routes::get_routes())
        .mount("/api", api::routes::get_routes())
        .attach(DBPool::fairing())
}

fn setup_db(db_url: &str) -> Figment {
    let db_url = env::var(db_url).unwrap();
    let db: Map<_, Value> = map! {
        "pool_size" => 10.into(),
        "url" => db_url.into(),
    };

    rocket::Config::figment().merge(("databases", map!["launchpad_db" => db]))
}
