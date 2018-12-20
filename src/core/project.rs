use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct ProjectId(Uuid);

impl ProjectId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for ProjectId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub description: Option<String>,
    pub status: ProjectStatus,
    #[serde(with = "url_serde")]
    pub url: Option<Url>,
    pub started_at: DateTime<Utc>,
}

/// The status of a [`Project`].
#[derive(Debug, Serialize, Deserialize)]
pub enum ProjectStatus {
    InProgress,
    Paused(DateTime<Utc>),
    Completed(DateTime<Utc>),
    Abandoned(DateTime<Utc>),
}
