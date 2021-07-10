use chrono::NaiveDate;
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ProjectBasic {
    pub end_date: Option<NaiveDate>,
    pub start_date: NaiveDate,
    pub title: String,
}

#[derive(Deserialize)]
pub struct StandardRequest<T> {
    pub data: T,
}

#[derive(Serialize)]
pub struct StandardResponse<T> {
    pub data: T,
}
