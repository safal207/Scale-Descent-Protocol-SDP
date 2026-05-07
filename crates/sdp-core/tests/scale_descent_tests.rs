use sdp_core::{
    collapse_hypotheses, validate_decision_sequence, AgentContract, Decision, Evidence,
    Hypothesis, Permission, ScaleLevel, SdpError,
};

fn evidence(id: &str, confidence: f32) -> Evidence {
    Evidence {
        evidence_id: id.to_string(),
        source: "test".to_string(),
        claim: "test claim".to_string(),
        confidence,
        scale: ScaleLevel::Pico,
    }
}

#[test]
fn execute_requires_commit() {
    let decisions = vec![Decision::Proceed, Decision::Hold, Decision::Execute];

    let result = validate_decision_sequence(&decisions);

    assert!(matches!(result, Err(SdpError::InvalidTransition(_))));
}

#[test]
fn execute_after_commit_is_valid() {
    let decisions = vec![Decision::Proceed, Decision::Commit, Decision::Execute];

    let result = validate_decision_sequence(&decisions);

    assert!(result.is_ok());
}

#[test]
fn commit_requires_evidence() {
    let hypotheses = vec![Hypothesis {
        id: "H1".to_string(),
        description: "unsupported but high-prior hypothesis".to_string(),
        prior: 0.9,
        evidence: vec![],
    }];

    let result = collapse_hypotheses(ScaleLevel::Pico, &hypotheses);

    assert!(matches!(result, Err(SdpError::CommitRequiresEvidence)));
}

#[test]
fn evidence_confidence_range() {
    let invalid = evidence("E1", 1.2);

    let result = invalid.validate();

    assert!(matches!(result, Err(SdpError::InvalidEvidenceConfidence(_))));
}

#[test]
fn collapse_selects_best_hypothesis() {
    let hypotheses = vec![
        Hypothesis {
            id: "H1".to_string(),
            description: "weak hypothesis".to_string(),
            prior: 0.1,
            evidence: vec![evidence("E1", 0.4)],
        },
        Hypothesis {
            id: "H2".to_string(),
            description: "strong hypothesis".to_string(),
            prior: 0.3,
            evidence: vec![evidence("E2", 0.9)],
        },
    ];

    let result = collapse_hypotheses(ScaleLevel::Micro, &hypotheses).unwrap();

    assert_eq!(result.selected_hypothesis_id, "H2");
    assert_eq!(result.decision, Decision::Commit);
}

#[test]
fn collapse_requires_micro_or_pico() {
    let hypotheses = vec![Hypothesis {
        id: "H1".to_string(),
        description: "macro-only hypothesis".to_string(),
        prior: 0.3,
        evidence: vec![evidence("E1", 0.8)],
    }];

    let result = collapse_hypotheses(ScaleLevel::Macro, &hypotheses);

    assert!(matches!(result, Err(SdpError::CollapseRequiresCausalDepth)));
}

#[test]
fn agent_execute_requires_permission() {
    let agent = AgentContract {
        agent_id: "micro-agent-001".to_string(),
        scale: ScaleLevel::Micro,
        mission: "inspect transition".to_string(),
        input_scope: vec!["trace:event-001".to_string()],
        permissions: vec![Permission::Read, Permission::Inspect],
        max_steps: 8,
        trace_required: true,
    };

    let result = agent.validate_execute_permission();

    assert!(matches!(result, Err(SdpError::ExecutePermissionMissing)));
}
