use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use axum_macros::debug_handler;
use futures_util::StreamExt;
use mongodb::{
    bson::{doc, Document},
    error::Error,
    Database,
};
use shared::{ProjectInfo, ProjectInfoPayload};
use std::sync::Arc;

const COLLECTION_NAME: &str = "projects";

pub async fn get_all(Extension(db): Extension<Arc<Database>>) -> impl IntoResponse {
    let cursor = db
        .collection::<ProjectInfo>(COLLECTION_NAME)
        .find(None, None)
        .await
        .unwrap();
    let projects: Vec<Result<ProjectInfo, Error>> = cursor.collect().await;
    let projects: Vec<ProjectInfo> = projects
        .into_iter()
        .filter(|r| matches!(r, Ok(_)))
        .map(|r| match r {
            Ok(pi) => pi,
            _ => ProjectInfo::default(),
        })
        .collect();

    (StatusCode::OK, Json(projects))
}

pub async fn get_one(
    Path(id): Path<usize>,
    Extension(db): Extension<Arc<Database>>,
) -> impl IntoResponse {
    let project_info = db
        .collection::<ProjectInfo>(COLLECTION_NAME)
        .find_one(doc! {"id": id as u32, }, None)
        .await
        .unwrap();

    if let Some(pi) = project_info {
        (StatusCode::OK, Json(Some(pi)))
    } else {
        (StatusCode::NOT_FOUND, Json(None))
    }
}

#[debug_handler]
pub async fn post(
    Json(payload): Json<ProjectInfoPayload>,
    Extension(db): Extension<Arc<Database>>,
) -> impl IntoResponse {
    let collection = db.collection::<ProjectInfo>(COLLECTION_NAME);

    let highest_ids = collection
        .aggregate(
            [doc! { "$group" : {"_id": "$id", "max": { "$max": "$id" }} }],
            None,
        )
        .await
        .unwrap();
    let highest_ids: Vec<Result<Document, Error>> = highest_ids.collect().await;

    if let Ok(doc) = &highest_ids[0] {
        let id: u32 = (doc.get("max").unwrap().as_i32().unwrap() + 1) as u32;

        log::debug!("Next ID will be {:?}", id);

        (
            StatusCode::CREATED,
            Json(Some(ProjectInfo {
                id,
                name: payload.name,
                link: payload.link,
                desc: payload.desc,
            })),
        )
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
    }
}

pub async fn put(Path(_id): Path<usize>) -> impl IntoResponse {
    todo!();
}

pub async fn delete(Path(_id): Path<usize>) -> impl IntoResponse {
    todo!();
}

/// Reset MongoDB Collection
pub async fn initialize(db: Arc<Database>) {
    let projects: [ProjectInfo; 2] = [
        ProjectInfo {
            id: 0,
            name: "DinoDungeons".to_string(),
            link: Some("http://github.com/LForchini/DinoDungeons".to_string()),
            desc:
                "A small game I made in a team of 4 during my apprenticeship in a 2 day hackathon."
                    .to_string(),
        },
        ProjectInfo {
            id: 1,
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

    let collection = db.collection::<ProjectInfo>(COLLECTION_NAME);
    collection.delete_many(doc! {}, None).await.unwrap();
    collection.insert_many(projects, None).await.unwrap();
}
