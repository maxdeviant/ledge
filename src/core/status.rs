use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};

use super::ProjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub sessions: Vec<Session>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    /// The ID of the project to which the session correponds.
    pub project_id: ProjectId,
    /// The timestamp of when the session was started.
    pub started_at: DateTime<Utc>,
}
