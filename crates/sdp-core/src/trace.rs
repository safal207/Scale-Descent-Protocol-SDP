use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{decision::Decision, scale::ScaleLevel};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceEvent {
    pub event_id: String,
    pub task_id: String,
    pub scale: ScaleLevel,
    pub decision: Decision,
    pub timestamp: DateTime<Utc>,
    pub evidence_ids: Vec<String>,
    pub reason: String,
}
