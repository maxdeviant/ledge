use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

use super::ProjectId;

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct EntryId(Uuid);

impl EntryId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for EntryId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub id: EntryId,

    #[serde(flatten)]
    pub kind: EntryKind,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum EntryKind {
    Event(EventEntry),
    ProjectHeadway(ProjectHeadwayEntry),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventEntry {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectHeadwayEntry {
    pub project_id: ProjectId,
    pub hours_spent: f64,
}
