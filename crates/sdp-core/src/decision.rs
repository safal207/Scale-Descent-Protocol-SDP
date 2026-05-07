use serde::{Deserialize, Serialize};

use crate::error::SdpError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Decision {
    Proceed,
    Hold,
    Reject,
    Commit,
    Execute,
}

pub fn validate_decision_sequence(decisions: &[Decision]) -> Result<(), SdpError> {
    let mut committed = false;

    for decision in decisions {
        match decision {
            Decision::Commit => committed = true,
            Decision::Execute if !committed => {
                return Err(SdpError::InvalidTransition(
                    "Execute requires prior Commit".to_string(),
                ));
            }
            _ => {}
        }
    }

    Ok(())
}
