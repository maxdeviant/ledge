use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct ProjectId(Uuid);

impl ProjectId {
    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for ProjectId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

#[derive(Debug)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub status: ProjectStatus,
    pub started_at: DateTime<Utc>,
}

/// The status of a [`Project`].
#[derive(Debug)]
pub enum ProjectStatus {
    InProgress,
    Paused(DateTime<Utc>),
    Completed(DateTime<Utc>),
    Abandoned(DateTime<Utc>),
}
