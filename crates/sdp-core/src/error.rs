use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum SdpError {
    #[error("invalid transition: {0}")]
    InvalidTransition(String),

    #[error("commit requires at least one evidence item")]
    CommitRequiresEvidence,

    #[error("evidence confidence must be between 0.0 and 1.0: {0}")]
    InvalidEvidenceConfidence(f32),

    #[error("collapse requires Micro or Pico causal depth")]
    CollapseRequiresCausalDepth,

    #[error("agent requires Execute permission before executing")]
    ExecutePermissionMissing,

    #[error("no hypotheses were provided")]
    NoHypotheses,
}
