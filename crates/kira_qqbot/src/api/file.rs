use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QQFile {
    pub id: String,
    pub name: String,
    pub size: i64,
    pub busid: i64,
}