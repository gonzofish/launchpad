use chrono::NaiveDate;
use rocket::{
    http::{ContentType, Status},
    local::blocking::Client,
};

use launchpad::models::{db_models, http_models};

type ProjectResponse = http_models::StandardRequest<db_models::Project>;

#[test]
fn test_create() {
    let client = blast_off();
    let new_project = r#"{
        "data": {
            "end_date": "2022-12-31",
            "start_date": "2022-01-01",
            "title": "Get in shape!"
        }
    }"#;
    let response = client
        .post("/api/project")
        .header(ContentType::JSON)
        .body(new_project)
        .dispatch();
    let status = response.status();
    let response_project = response.into_json::<ProjectResponse>();

    assert_eq!(status, Status::Created);

    if let Some(project_json) = response_project {
        let project = project_json.data;

        assert_eq!(project.id, 1);
        assert_eq!(project.end_date, NaiveDate::from_ymd_opt(2022, 12, 31));
        assert_eq!(project.start_date, NaiveDate::from_ymd(2022, 1, 1));
        assert_eq!(project.title, String::from("Get in shape!"));
    } else {
        panic!("Response did not contain a project");
    }
}

fn blast_off() -> Client {
    let test_instance = launchpad::blast_off("DATABASE_URL");

    Client::tracked(test_instance).expect("Rocket instance is invalid")
}
