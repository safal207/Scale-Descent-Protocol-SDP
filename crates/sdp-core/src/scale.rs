use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScaleLevel {
    Macro,
    Meso,
    Micro,
    Pico,
    Collapse,
}

impl ScaleLevel {
    pub fn is_causal_depth(self) -> bool {
        matches!(self, ScaleLevel::Micro | ScaleLevel::Pico)
    }
}
