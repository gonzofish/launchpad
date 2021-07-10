use rocket::Route;

use diesel::prelude::*;
use rocket::response::status::Created;
use rocket::serde::json::Json;

use crate::error::CustomError;
use crate::http_models;
use crate::models::{NewProject, Project};
use crate::schema;
use crate::DBPool;

type ProjectRequest = http_models::ProjectRequest;
type ProjectResponse = http_models::ProjectResponse;
type ProjectListResponse = http_models::ProjectListResponse;

pub fn get_routes() -> Vec<Route> {
    routes![
        create_project,
        get_all_projects,
        get_one_project,
        update_project
    ]
}

#[post("/project", format = "json", data = "<body>")]
async fn create_project(
    conn: DBPool,
    body: Json<ProjectRequest>,
) -> Result<Created<Json<ProjectResponse>>, CustomError> {
    let new_project: Project = conn
        .run(move |c| {
            diesel::insert_into(schema::projects::table)
                .values(NewProject {
                    end_date: body.data.end_date,
                    start_date: body.data.start_date,
                    title: &body.data.title,
                })
                .get_result(c)
        })
        .await?;
    let response = ProjectResponse { data: new_project };

    Ok(Created::new("/project").body(Json(response)))
}

#[put("/project/<id>", format = "json", data = "<body>")]
async fn update_project(
    conn: DBPool,
    body: Json<ProjectRequest>,
    id: i32,
) -> Result<Json<ProjectResponse>, CustomError> {
    use schema::projects::dsl::projects;

    let response: Project = conn
        .run(move |c| {
            diesel::update(projects.find(id))
                .set(NewProject {
                    end_date: body.data.end_date,
                    start_date: body.data.start_date,
                    title: &body.data.title,
                })
                .get_result(c)
        })
        .await?;

    Ok(Json(ProjectResponse { data: response }))
}

#[get("/project")]
async fn get_all_projects(conn: DBPool) -> Result<Json<ProjectListResponse>, CustomError> {
    let projects: Vec<Project> = conn
        .run(move |c| schema::projects::table.get_results(c))
        .await?;

    Ok(Json(ProjectListResponse { data: projects }))
}

#[get("/project/<id>")]
async fn get_one_project(conn: DBPool, id: i32) -> Result<Json<ProjectResponse>, CustomError> {
    let project: Project = conn
        .run(move |c| schema::projects::table.find(id).get_result(c))
        .await?;
    let response = ProjectResponse { data: project };

    Ok(Json(response))
}
