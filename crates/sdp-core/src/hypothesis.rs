use serde::{Deserialize, Serialize};

use crate::Evidence;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hypothesis {
    pub id: String,
    pub description: String,
    pub prior: f32,
    pub evidence: Vec<Evidence>,
}

impl Hypothesis {
    pub fn score(&self) -> f32 {
        let evidence_average = if self.evidence.is_empty() {
            0.0
        } else {
            self.evidence.iter().map(|item| item.confidence).sum::<f32>()
                / self.evidence.len() as f32
        };

        (self.prior + evidence_average).clamp(0.0, 1.0)
    }
}
