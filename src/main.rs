#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

mod models;
mod schema;
#[cfg(test)]
mod tests;

pub type DBPool = launchpad::DBPool;

#[launch]
fn rocket() -> _ {
    launchpad::blast_off("DATABASE_URL")
}
