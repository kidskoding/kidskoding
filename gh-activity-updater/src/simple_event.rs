use serde::Deserialize;

use crate::repo::Repo;

#[derive(Debug, Deserialize)]
pub struct SimpleEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub repo: Repo,
}