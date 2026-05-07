use serde::{Deserialize, Serialize};

use crate::{decision::Decision, error::SdpError, hypothesis::Hypothesis, scale::ScaleLevel};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollapseResult {
    pub selected_hypothesis_id: String,
    pub confidence: f32,
    pub decision: Decision,
    pub reason: String,
}

pub fn collapse_hypotheses(
    current_scale: ScaleLevel,
    hypotheses: &[Hypothesis],
) -> Result<CollapseResult, SdpError> {
    if !current_scale.is_causal_depth() {
        return Err(SdpError::CollapseRequiresCausalDepth);
    }

    let selected = hypotheses
        .iter()
        .max_by(|left, right| left.score().total_cmp(&right.score()))
        .ok_or(SdpError::NoHypotheses)?;

    for evidence in selected.evidence.iter() {
        evidence.validate()?;
    }

    let confidence = selected.score();
    let decision = if confidence >= 0.75 {
        Decision::Commit
    } else if confidence >= 0.4 {
        Decision::Hold
    } else {
        Decision::Reject
    };

    if decision == Decision::Commit && selected.evidence.is_empty() {
        return Err(SdpError::CommitRequiresEvidence);
    }

    Ok(CollapseResult {
        selected_hypothesis_id: selected.id.clone(),
        confidence,
        decision,
        reason: "Selected highest-scoring hypothesis after evidence collapse".to_string(),
    })
}
