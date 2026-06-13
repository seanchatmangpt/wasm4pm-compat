//! Process Mining/Workflow witness markers — compiled from `ontology/witnesses-workflow.ttl`.
//!
//! Every entry here is derived from a `compat:WitnessMarker` tagged
//! `compat:paperCategory "workflow"` in the ontology.
//!
//! To add a paper: declare a `compat:WitnessMarker` with
//! `compat:paperCategory "workflow"` in
//! `ontology/witnesses-workflow.ttl`, then run
//! `cargo make ggen-witnesses-workflow`.
//!
//! Structure-only authority labels. Graduate to `wasm4pm` when the
//! paper's algorithm must actually execute.

use crate::witness::WitnessFamily;

witness_marker!(
    /// Compliance-aware PPM — De Santis, Park, van der Aalst & Zanichelli (2026): neuro-symbolic monitoring.
    ComplianceAwarePredictivePaper,
    "workflow/compliance-aware-predictive-2026",
    WitnessFamily::Paper,
    "Compliance-Aware Predictive Process Monitoring: A Neuro-Symbolic Approach",
    Some(2026)
);

witness_marker!(
    /// Practical guide: Python automation techniques for DevOps workflows.
    HandsonPythonDevopsPaper,
    "workflow/handson-python-devops",
    WitnessFamily::Paper,
    "Hands-On Python for DevOps",
    None
);

witness_marker!(
    /// Separable WF-nets (duplicate entry for PDF in workflow/) — Kourani, Park & van der Aalst (2026).
    HierarchicalSeparableWfNetPaper,
    "workflow/hierarchical-separable-wfnet-2026",
    WitnessFamily::Paper,
    "Hierarchical Decomposition of Separable Workflow-Nets",
    Some(2026)
);

witness_marker!(
    /// Anthropic internal practices for Claude Code in software development workflows.
    HowAnthropicUsesClaudeCodePaper,
    "workflow/anthropic-claude-code-usage",
    WitnessFamily::Paper,
    "How Anthropic Teams Use Claude Code",
    None
);

witness_marker!(
    /// OCEL 2.0 specification paper (arXiv:2403.01975) — companion to the compat:Ocel20 standard witness.
    Ocel20SpecPaper,
    "workflow/ocel-2-0-spec-2024",
    WitnessFamily::Standard,
    "OCEL 2.0 Specification",
    Some(2024)
);

witness_marker!(
    /// OCPQ PDF copy in workflow/ — Küsters & van der Aalst (2025).
    OcpqPaperWorkflow,
    "workflow/ocpq-paper-2025",
    WitnessFamily::Paper,
    "OCPQ: Object-Centric Process Querying and Constraints",
    Some(2025)
);

witness_marker!(
    /// PM4Py library paper (main reference) — Berti, van Zelst & Schuster: comprehensive PM4Py description.
    Pm4pyLibraryPaper,
    "workflow/pm4py-library-python",
    WitnessFamily::Paper,
    "PM4Py: A Process Mining Library for Python",
    None
);

witness_marker!(
    /// PM4Py Software Impacts paper — Berti, van Zelst & Schuster (2023): software description.
    Pm4pySoftwareImpactsPaper,
    "workflow/pm4py-software-impacts-2023",
    WitnessFamily::Paper,
    "PM4Py: A Process Mining Library for Python (Software Impacts)",
    Some(2023)
);

witness_marker!(
    /// PMAx — Antonov, Kourani, Berti, Park & van der Aalst (2026): AI-driven process mining agents.
    PmaxAgenticFrameworkPaper,
    "workflow/pmax-agentic-process-mining-2026",
    WitnessFamily::Paper,
    "PMAx: An Agentic Framework for AI-Driven Process Mining",
    Some(2026)
);

witness_marker!(
    /// PM in healthcare — Munoz-Gama et al. (2022): survey of process mining applied to clinical workflows.
    ProcessMiningHealthcarePaper,
    "workflow/process-mining-healthcare-2022",
    WitnessFamily::Paper,
    "Process Mining for Healthcare: Characteristics and Challenges",
    Some(2022)
);

witness_marker!(
    /// Real-Life BPMN 4th ed. — Freund & Rücker: practical BPMN guide with real-world examples.
    RealLifeBpmn4Paper,
    "workflow/real-life-bpmn-edition-4",
    WitnessFamily::Paper,
    "Real-Life BPMN (4th edition)",
    None
);

witness_marker!(
    /// sAirflow: serverless adaptation of Apache Airflow workflow scheduling architecture.
    SAirflowServerlessPaper,
    "workflow/sairflow-serverless-scheduler",
    WitnessFamily::Paper,
    "sAirflow: Adopting Serverless in a Legacy Workflow Scheduler",
    None
);

witness_marker!(
    /// Stochastic Conformance — Leemans, Syring & van der Aalst (2020): fitness/precision on stochastic nets.
    StochasticConformancePaper,
    "workflow/stochastic-conformance-2020",
    WitnessFamily::Paper,
    "Stochastic Conformance Checking: Comparing Stochastic Process Models and Event Logs",
    Some(2020)
);

witness_marker!(
    /// Why Automate?: societal implications of robot automation on well-being across social groups.
    WhyAutomateThisPaper,
    "workflow/why-automate-time-use-wellbeing",
    WitnessFamily::Paper,
    "Why Automate This? Exploring the Connection between Time Use, Well-being and Robot Automation",
    None
);

witness_marker!(
    /// Workflow Patterns Definitive Guide (PDF copy) — Russell, van der Aalst & ter Hofstede (2016).
    WorkflowPatternDefinitiveGuidePaper,
    "workflow/workflow-patterns-definitive-guide-2016",
    WitnessFamily::Paper,
    "Workflow Patterns: The Definitive Guide",
    Some(2016)
);

witness_marker!(
    /// Workflows Community Summit 2024: community consensus on scientific workflow challenges.
    WorkflowsCommunitySubmit2024Paper,
    "workflow/workflows-community-summit-2024",
    WitnessFamily::Paper,
    "Workflows Community Summit 2024: Future Trends and Challenges in Scientific Workflows",
    Some(2024)
);

witness_marker!(
    /// XES→OCED integration — Latif, Latif & Rahman (2025): bridging XES and object-centric modeling.
    XesToOcedIntegrationPaper,
    "workflow/xes-oced-integration-2025",
    WitnessFamily::Paper,
    "Object-Centric Analysis of XES Event Logs: Integrating OCED Modeling with SPARQL Queries",
    Some(2025)
);

witness_marker!(
    /// YAWL open-source BPMS — van der Aalst & ter Hofstede: YAWL system and engine description.
    YawlOpenSourceBpmsPaper,
    "workflow/yawl-open-source-bpms",
    WitnessFamily::Paper,
    "YAWL: An Open Source Business Process Management System",
    None
);

witness_marker!(
    /// YAWL 5.0 Technical Manual: engine internals, deployment, and extension points.
    YawlTechnicalManual50Paper,
    "workflow/yawl-technical-manual-5-0",
    WitnessFamily::Paper,
    "YAWL Technical Manual 5.0",
    None
);

witness_marker!(
    /// YAWL 5.1 User Manual: end-user guide for YAWL workflow authoring and monitoring.
    YawlUserManual51Paper,
    "workflow/yawl-user-manual-5-1",
    WitnessFamily::Paper,
    "YAWL User Manual 5.1",
    None
);

witness_marker!(
    /// YAWL original paper — van der Aalst & ter Hofstede (2004): YAWL semantics and expressiveness.
    YawlYetAnotherWorkflowLanguagePaper,
    "workflow/yawl-yet-another-workflow-language-2004",
    WitnessFamily::Paper,
    "YAWL: Yet Another Workflow Language",
    Some(2004)
);
