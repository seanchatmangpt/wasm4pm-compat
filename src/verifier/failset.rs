//! Residual-failset shapes — which checks failed, as inert structured data.
//!
//! ## What this module IS
//!
//! - Serializable containers naming the compliance checks that did not pass and
//!   the residual evidence left after a replay verification.
//!
//! ## What this module is **NOT**
//!
//! - **Not** a checker. It evaluates no rule and replays no log; it only holds
//!   the failset another (graduated) engine produced.
//!
//! Structure only.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ResidualFailset {
    pub unreplayable_transitions: Vec<FailedTransition>,
    pub deadlocked_places: Vec<String>,
    pub residual_marking: HashMap<String, u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct FailedTransition {
    pub transition_id: String,
    pub missing_places: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticPayload {
    pub code: String,
    pub message: String,
}

pub trait ComplianceDiagnostic {
    fn diagnostic_payload(&self) -> DiagnosticPayload;
}

impl ComplianceDiagnostic for FailedTransition {
    fn diagnostic_payload(&self) -> DiagnosticPayload {
        DiagnosticPayload {
            code: "W4PM-010-FAIL-UNREPLAYABLE".to_string(),
            message: format!(
                "Transition {} could not be replayed due to missing tokens in places: {:?}",
                self.transition_id, self.missing_places
            ),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct VerificationReport {
    pub is_compliant: bool,
    pub failset: Option<ResidualFailset>,
}
