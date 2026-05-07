use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::scale::ScaleLevel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SdpTask {
    pub task_id: String,
    pub goal: String,
    pub required_scale: ScaleLevel,
    pub context: String,
    pub created_at: DateTime<Utc>,
}
