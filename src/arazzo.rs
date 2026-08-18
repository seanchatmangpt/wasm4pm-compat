//! Arazzo 1.1 structural compatibility types.
//!
//! This module is intentionally structure-only. It models the public Arazzo
//! document surface consumed by wasm4pm. Parsing, reference resolution,
//! admission, criterion evaluation, and execution belong to wasm4pm.
//!
//! Specification authority:
//! <https://spec.openapis.org/arazzo/v1.1.0.html>

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A complete Arazzo 1.1 description.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArazzoDescription {
    /// Arazzo specification version, e.g. `1.1.0`.
    pub arazzo: String,

    /// Optional self-assigned URI used for identity and base-URI resolution.
    #[serde(rename = "$self", skip_serializing_if = "Option::is_none")]
    pub self_uri: Option<String>,

    pub info: ArazzoInfo,

    pub source_descriptions: Vec<SourceDescription>,

    pub workflows: Vec<Workflow>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Components>,

    /// Arazzo specification extensions (`x-*`).
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extensions: BTreeMap<String, Value>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArazzoInfo {
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub version: String,

    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extensions: BTreeMap<String, Value>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceDescription {
    pub name: String,

    pub url: String,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub source_type: Option<SourceDescriptionType>,

    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extensions: BTreeMap<String, Value>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SourceDescriptionType {
    Openapi,
    Asyncapi,
    Arazzo,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Workflow {
    pub workflow_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// JSON Schema 2020-12 object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Value>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<String>,

    pub steps: Vec<Step>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub success_actions: Vec<SuccessActionOrReference>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub failure_actions: Vec<FailureActionOrReference>,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub outputs: BTreeMap<String, OutputValue>,

    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extensions: BTreeMap<String, Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Step {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub step_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ParameterOrReference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<RequestBody>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub success_criteria: Vec<Criterion>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub on_success: Vec<SuccessActionOrReference>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub on_failure: Vec<FailureActionOrReference>,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub outputs: BTreeMap<String, OutputValue>,

    /// Maximum duration in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<AsyncAction>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<String>,

    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extensions: BTreeMap<String, Value>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AsyncAction {
    Send,
    Receive,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParameterOrReference {
    Parameter(Parameter),
    Reference(ReusableObject),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    pub name: String,

    #[serde(rename = "in", skip_serializing_if = "Option::is_none")]
    pub location: Option<ParameterLocation>,

    pub value: ParameterValue,

    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extensions: BTreeMap<String, Value>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParameterLocation {
    Path,
    Query,
    Querystring,
    Header,
    Cookie,
}

/// Literal, runtime expression, or selector-shaped parameter value.
pub type ParameterValue = Value;

/// Runtime expression string or Selector Object.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OutputValue {
    Expression(String),
    Selector(Selector),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SuccessActionOrReference {
    Action(SuccessAction),
    Reference(ReusableObject),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuccessAction {
    pub name: String,

    #[serde(rename = "type")]
    pub action_type: SuccessActionType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_id: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ParameterOrReference>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub criteria: Vec<Criterion>,

    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extensions: BTreeMap<String, Value>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SuccessActionType {
    End,
    Goto,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FailureActionOrReference {
    Action(FailureAction),
    Reference(ReusableObject),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FailureAction {
    pub name: String,

    #[serde(rename = "type")]
    pub action_type: FailureActionType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_id: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ParameterOrReference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_limit: Option<u64>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub criteria: Vec<Criterion>,

    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extensions: BTreeMap<String, Value>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FailureActionType {
    End,
    Retry,
    Goto,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Components {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub inputs: BTreeMap<String, Value>,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub parameters: BTreeMap<String, Parameter>,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub success_actions: BTreeMap<String, SuccessAction>,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub failure_actions: BTreeMap<String, FailureAction>,

    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extensions: BTreeMap<String, Value>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReusableObject {
    pub reference: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Criterion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,

    pub condition: String,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub expression_type: Option<ExpressionTypeOrKind>,

    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extensions: BTreeMap<String, Value>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExpressionTypeOrKind {
    Kind(ExpressionKind),
    Versioned(ExpressionType),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExpressionKind {
    Simple,
    Regex,
    Jsonpath,
    Xpath,
    Jsonpointer,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpressionType {
    #[serde(rename = "type")]
    pub expression_type: SelectorKind,

    pub version: String,

    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extensions: BTreeMap<String, Value>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SelectorKind {
    Jsonpath,
    Xpath,
    Jsonpointer,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Selector {
    pub context: String,

    pub selector: String,

    #[serde(rename = "type")]
    pub selector_type: SelectorType,

    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extensions: BTreeMap<String, Value>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SelectorType {
    Kind(SelectorKind),
    Versioned(ExpressionType),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub replacements: Vec<PayloadReplacement>,

    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extensions: BTreeMap<String, Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayloadReplacement {
    pub target: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_selector_type: Option<SelectorType>,

    pub value: Value,

    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extensions: BTreeMap<String, Value>,
}
