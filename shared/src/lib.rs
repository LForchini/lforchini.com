use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct ProjectInfoPayload {
    pub name: String,
    pub link: Option<String>,
    pub desc: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ProjectInfo {
    pub id: u32,
    pub name: String,
    pub link: Option<String>,
    pub desc: String,
}
