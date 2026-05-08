use serde::{Deserialize, Serialize};

use crate::{Hypothesis, ScaleLevel, SdpTask};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescentRun {
    pub task: SdpTask,
    pub current_scale: ScaleLevel,
    pub hypotheses: Vec<Hypothesis>,
}
