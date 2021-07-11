use crate::schema::projects;
use chrono::NaiveDate;
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Queryable, Serialize)]
pub struct Project {
    pub id: i32,
    pub end_date: Option<NaiveDate>,
    pub start_date: NaiveDate,
    pub title: String,
}

#[derive(AsChangeset, Insertable)]
#[table_name = "projects"]
pub struct NewProject<'a> {
    pub end_date: Option<NaiveDate>,
    pub start_date: NaiveDate,
    pub title: &'a str,
}
