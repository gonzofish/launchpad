use rocket::Route;

use diesel::prelude::*;
use rocket::response::status::Created;
use rocket::serde::json::Json;

use crate::error::CustomError;
use crate::models::db_models;
use crate::models::http_models;
use crate::schema;
use crate::DBPool;

type ProjectRequest = http_models::StandardRequest<http_models::ProjectBasic>;
type ProjectResponse = http_models::StandardResponse<db_models::Project>;
type ProjectListResponse = http_models::StandardResponse<Vec<db_models::Project>>;

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
    let new_project: db_models::Project = conn
        .run(move |c| {
            diesel::insert_into(schema::projects::table)
                .values(db_models::NewProject {
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

    let response: db_models::Project = conn
        .run(move |c| {
            diesel::update(projects.find(id))
                .set(db_models::NewProject {
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
    let projects: Vec<db_models::Project> = conn
        .run(move |c| schema::projects::table.get_results(c))
        .await?;

    Ok(Json(ProjectListResponse { data: projects }))
}

#[get("/project/<id>")]
async fn get_one_project(conn: DBPool, id: i32) -> Result<Json<ProjectResponse>, CustomError> {
    let project: db_models::Project = conn
        .run(move |c| schema::projects::table.find(id).get_result(c))
        .await?;
    let response = ProjectResponse { data: project };

    Ok(Json(response))
}
