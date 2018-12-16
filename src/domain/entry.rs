use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

use super::ProjectId;

#[derive(Debug)]
pub struct EntryId(Uuid);

#[derive(Debug)]
pub struct Entry {
    pub id: EntryId,
    pub kind: EntryKind,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug)]
pub enum EntryKind {
    Event(EventEntry),
    ProjectHeadway(ProjectHeadwayEntry),
}

#[derive(Debug)]
pub struct EventEntry {
    pub name: String,
}

#[derive(Debug)]
pub struct ProjectHeadwayEntry {
    pub project_id: ProjectId,
    pub time_spent: Duration,
}
