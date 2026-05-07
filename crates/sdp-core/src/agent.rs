use serde::{Deserialize, Serialize};

use crate::{error::SdpError, scale::ScaleLevel};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Permission {
    Read,
    Inspect,
    Simulate,
    ProposePatch,
    Execute,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentContract {
    pub agent_id: String,
    pub scale: ScaleLevel,
    pub mission: String,
    pub input_scope: Vec<String>,
    pub permissions: Vec<Permission>,
    pub max_steps: u32,
    pub trace_required: bool,
}

impl AgentContract {
    pub fn can_execute(&self) -> bool {
        self.permissions.contains(&Permission::Execute)
    }

    pub fn validate_execute_permission(&self) -> Result<(), SdpError> {
        if self.can_execute() {
            Ok(())
        } else {
            Err(SdpError::ExecutePermissionMissing)
        }
    }
}
