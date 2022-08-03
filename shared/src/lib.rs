use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Project {
    pub name: String,
    pub link: Option<String>,
    pub desc: String,
}
