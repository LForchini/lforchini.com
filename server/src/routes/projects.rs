use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use axum_macros::debug_handler;
use futures_util::TryStreamExt;
use mongodb::{bson::doc, error::Error, Collection};
use shared::Project;

#[debug_handler]
pub async fn get_projects(
    Extension(collection): Extension<Collection<Project>>,
) -> impl IntoResponse {
    let cursor = collection.find(None, None).await.unwrap();
    let projects: Result<Vec<Project>, Error> = cursor.try_collect().await;

    match projects {
        Ok(p) => (StatusCode::OK, Json(Some(p))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}

#[debug_handler]
pub async fn add_project(
    Json(project): Json<Project>,
    Extension(collection): Extension<Collection<Project>>,
) -> impl IntoResponse {
    let result = collection.insert_one(project, None).await;
    match result {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

/// Reset MongoDB Collection
pub async fn initialize(collection: Collection<Project>) {
    let projects: [Project; 2] = [
        Project {
            name: "DinoDungeons".to_string(),
            link: Some("http://github.com/LForchini/DinoDungeons".to_string()),
            desc:
                "A small game I made in a team of 4 during my apprenticeship in a 2 day hackathon."
                    .to_string(),
        },
        Project {
            name: "lforchini.com".to_string(),
            link: Some("http://github.com/LForchini/lforchini.com".to_string()),
            desc: "This personal website made entirely in Rust.".to_string(),
        },
    ];

    log::warn!(
        "Clearning `projects` collection and inserting {:?} initial values",
        projects.len()
    );
    log::debug!("Inserting {:?}", projects);

    collection.delete_many(doc! {}, None).await.unwrap();
    collection.insert_many(projects, None).await.unwrap();
}
