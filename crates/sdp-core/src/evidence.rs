use serde::{Deserialize, Serialize};

use crate::{error::SdpError, scale::ScaleLevel};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub evidence_id: String,
    pub source: String,
    pub claim: String,
    pub confidence: f32,
    pub scale: ScaleLevel,
}

impl Evidence {
    pub fn validate(&self) -> Result<(), SdpError> {
        if !(0.0..=1.0).contains(&self.confidence) {
            return Err(SdpError::InvalidEvidenceConfidence(self.confidence));
        }

        Ok(())
    }
}
