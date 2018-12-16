use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct ProjectId(Uuid);

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
