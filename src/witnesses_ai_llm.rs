












//! AI/LLM/Agents witness markers — compiled from `ontology/witnesses-ai-llm.ttl`.
//!
//! Every entry here is derived from a `compat:WitnessMarker` tagged
//! `compat:paperCategory "ai-llm"` in the ontology.
//!
//! To add a paper: declare a `compat:WitnessMarker` with
//! `compat:paperCategory "ai-llm"` in
//! `ontology/witnesses-ai-llm.ttl`, then run
//! `cargo make ggen-witnesses-ai-llm`.
//!
//! Structure-only authority labels. Graduate to `wasm4pm` when the
//! paper's algorithm must actually execute.


use crate::witness::WitnessFamily;


witness_marker!(
    /// CBR foundations (Aamodt & Plaza 1994) — PDF in AI_LLM/, breed witness in cognition.
    AamodtPlazaCbrAiLlmPaper,
    "ai-llm/aamodt-plaza-1994-cbr",
    WitnessFamily::Paper,
    "Case-Based Reasoning: Foundational Issues, Methodological Variations, and System Approaches (Aamodt & Plaza 1994)",
    Some(1994)
);

witness_marker!(
    /// Agent-based mathematical model for post-irradiation cellular response in digital twin.
    AgentBasedRadiationCellPaper,
    "ai-llm/agent-based-post-irradiation-cell-digital-twin",
    WitnessFamily::Paper,
    "Developing an Agent-Based Mathematical Model for Simulating Post-Irradiation Cellular Response",
    None
);

witness_marker!(
    /// Comprehensive review of agent-centric operating system architectures.
    AgentCentricOsPaper,
    "ai-llm/agent-centric-os-review",
    WitnessFamily::Paper,
    "Agent Centric Operating System: A Comprehensive Review and Outlook for Operating System",
    None
);

witness_marker!(
    /// TCP/IP-inspired agent-to-agent transaction protocol for multi-agent coordination.
    AgentTcpIpPaper,
    "ai-llm/agent-tcp-ip-transaction-system",
    WitnessFamily::Paper,
    "Agent TCP/IP: An Agent-to-Agent Transaction System",
    None
);

witness_marker!(
    /// Agentic AI framework for intent-based industrial automation control.
    AgenticAiIndustrialPaper,
    "ai-llm/agentic-ai-intent-industrial-automation",
    WitnessFamily::Paper,
    "Agentic AI for Intent-Based Industrial Automation",
    None
);

witness_marker!(
    /// Grow-and-refine multimodal semantic memory for agentic continual learning.
    AgenticLearnerGrowRefinePaper,
    "ai-llm/agentic-learner-grow-refine-semantic-memory",
    WitnessFamily::Paper,
    "Agentic Learner with Grow-and-Refine Multimodal Semantic Memory",
    None
);

witness_marker!(
    /// Field study of AI applications in environmental protection, Wisconsin context.
    AiEnvironmentalProtectionPaper,
    "ai-llm/ai-environmental-protection-wisconsin",
    WitnessFamily::Paper,
    "Artificial Intelligence in Environmental Protection: A Field Study in Wisconsin",
    None
);

witness_marker!(
    /// Protocol extension for AI multi-agent interoperability in multiparty conversation management.
    AiMultiAgentInteropPaper,
    "ai-llm/ai-multi-agent-interoperability-conversations",
    WitnessFamily::Paper,
    "AI Multi-Agent Interoperability Extension for Managing Multiparty Conversations",
    None
);

witness_marker!(
    /// AI-driven tailoring: feature influence on fashion product popularity prediction.
    AiTailoringFashionPaper,
    "ai-llm/ai-tailoring-fashion-image-popularity",
    WitnessFamily::Paper,
    "AI Tailoring: Evaluating Influence of Image Features on Fashion Product Popularity",
    None
);

witness_marker!(
    /// AI-augmented frameworks for human team formation, simulation, and optimization.
    AiTeamingPaper,
    "ai-llm/ai-era-teaming-frameworks-optimizing",
    WitnessFamily::Paper,
    "Teaming in the AI Era: AI-Augmented Frameworks for Forming, Simulating, and Optimizing Human Teams",
    None
);

witness_marker!(
    /// AlphaEvolve: Gemini-powered evolutionary agent for autonomous algorithm discovery.
    AlphaEvolvePaper,
    "ai-llm/alphaevolve-evolutionary-coding-agent",
    WitnessFamily::Paper,
    "AlphaEvolve: A Gemini-based Evolutionary Coding Agent",
    None
);

witness_marker!(
    /// AlphaStar Unplugged: large-scale offline RL for StarCraft II mastery.
    AlphaStarUnpluggedPaper,
    "ai-llm/alphastar-unplugged-offline-rl",
    WitnessFamily::Paper,
    "AlphaStar Unplugged: Large-Scale Offline Reinforcement Learning",
    None
);

witness_marker!(
    /// APRMCTS: MCTS-guided iterative tree search for LLM-based automated program repair.
    AprMctsPaper,
    "ai-llm/aprmcts-llm-program-repair-tree-search",
    WitnessFamily::Paper,
    "APRMCTS: Improving LLM-based Automated Program Repair with Iterative Tree Search",
    None
);

witness_marker!(
    /// AutoAgent: zero-code fully-automated framework for deploying LLM agent workflows.
    AutoAgentFrameworkPaper,
    "ai-llm/autoagent-zero-code-llm-agents",
    WitnessFamily::Paper,
    "AutoAgent: A Fully-Automated and Zero-Code Framework for LLM Agents",
    None
);

witness_marker!(
    /// GEPA-trained LLM framework for automated risk-of-bias assessment of RCTs.
    AutomatedRiskOfBiasPaper,
    "ai-llm/automated-risk-of-bias-gepa-programmatic",
    WitnessFamily::Paper,
    "Automated Risk-of-Bias Assessment of Randomized Controlled Trials: A GEPA-trained Programmatic Prompting Framework",
    None
);

witness_marker!(
    /// Shannon framework extension from communication theory to computing performance analysis.
    BackToBitsShannonPaper,
    "ai-llm/back-to-bits-shannon-computing-performance",
    WitnessFamily::Paper,
    "Back to Bits: Extending Shannon's Communication Performance Framework to Computing",
    None
);

witness_marker!(
    /// Bench-CoE: benchmark-driven expert collaboration framework for LLM ensembles.
    BenchCoEExpertCollaborationPaper,
    "ai-llm/bench-coe-expert-collaboration-benchmark",
    WitnessFamily::Paper,
    "Bench-CoE: a Framework for Collaboration of Experts from Benchmark",
    None
);

witness_marker!(
    /// Object-level planning bootstrapped from LLM world knowledge for robotics tasks.
    BootstrappingObjectPlanningPaper,
    "ai-llm/bootstrapping-object-planning-llm",
    WitnessFamily::Paper,
    "Bootstrapping Object-level Planning with Large Language Models",
    None
);

witness_marker!(
    /// LLM agents bridging the last-mile gap in time series forecasting pipelines.
    BridgingLastMileForecastingPaper,
    "ai-llm/bridging-last-mile-time-series-llm",
    WitnessFamily::Paper,
    "Bridging the Last Mile of Time Series Forecasting with LLM Agents",
    None
);

witness_marker!(
    /// Form-based UI design principles applied to conversational agent interaction design.
    BridgingUiChatbotPaper,
    "ai-llm/bridging-ui-chatbot-form-conversational",
    WitnessFamily::Paper,
    "Bridging UI Design and Chatbot Interactions: Applying Form-Based Principles to Conversational Agents",
    None
);

witness_marker!(
    /// CARJAN: AJAN-based agent framework for traffic scenario generation and simulation.
    CarjanTrafficScenarioPaper,
    "ai-llm/carjan-agent-traffic-scenario-ajan",
    WitnessFamily::Paper,
    "CARJAN: Agent-Based Generation and Simulation of Traffic Scenarios with AJAN",
    None
);

witness_marker!(
    /// Curated survey of classical AI foundational papers in AI_LLM/ directory.
    ClassicalAiFoundationsMdPaper,
    "ai-llm/classical-ai-foundations-overview",
    WitnessFamily::Paper,
    "Classical AI Foundations (Survey Document)",
    None
);

witness_marker!(
    /// Anthropic guide: best practices for using Claude Code in agentic software development.
    ClaudeCodeBestPracticesPaper,
    "ai-llm/claude-code-best-practices-agentic",
    WitnessFamily::Paper,
    "Claude Code: Best Practices for Agentic Coding",
    None
);

witness_marker!(
    /// CodeHalu: taxonomy and benchmarking of LLM code hallucinations via execution verification.
    CodeHaluPaper,
    "ai-llm/codehalu-llm-hallucination-execution",
    WitnessFamily::Paper,
    "CodeHalu: Investigating Code Hallucinations in LLMs via Execution-based Verification",
    None
);

witness_marker!(
    /// Empirical evaluation of LLM comprehension and application of software design patterns.
    CodeLlmDesignPatternsPaper,
    "ai-llm/code-llm-design-patterns-understanding",
    WitnessFamily::Paper,
    "Do Code LLMs Understand Design Patterns?",
    None
);

witness_marker!(
    /// CodeMirage: study of hallucination types in LLM-generated code.
    CodeMiragePaper,
    "ai-llm/codemirage-llm-code-hallucinations",
    WitnessFamily::Paper,
    "CodeMirage: Hallucinations in Code Generated by Large Language Models",
    None
);

witness_marker!(
    /// Retrospective on computational biology modelling tools and visions from past to present.
    ComputationalBioModellingPaper,
    "ai-llm/computational-bio-modelling-then-now",
    WitnessFamily::Paper,
    "Computational Modelling of Biological Systems Now and Then: Revisiting Tools and Visions",
    None
);

witness_marker!(
    /// ML crowd density classification for Hajj safety management from video frames.
    CrowdDensityClassificationPaper,
    "ai-llm/ml-crowd-density-hajj-classification",
    WitnessFamily::Paper,
    "A Machine Learning Model for Crowd Density Classification in Hajj Video Frames",
    None
);

witness_marker!(
    /// CySecBench: cybersecurity-focused prompt dataset for LLM security benchmarking.
    CySecBenchPaper,
    "ai-llm/cysecbench-generative-ai-cybersecurity-prompts",
    WitnessFamily::Paper,
    "CySecBench: Generative AI-based CyberSecurity-focused Prompt Dataset for Benchmarking Large Language Models",
    None
);

witness_marker!(
    /// Using Dafny as verification-aware IL between LLM code generation and execution.
    DafnyVerificationPaper,
    "ai-llm/dafny-verification-intermediate-code-gen",
    WitnessFamily::Paper,
    "Dafny as Verification-Aware Intermediate Language for Code Generation",
    None
);

witness_marker!(
    /// AI-guided data-driven preliminary design for electrical machines.
    DataDrivenElectricalMachinePaper,
    "ai-llm/data-driven-electrical-machine-design-ai",
    WitnessFamily::Paper,
    "Data Driven Automatic Electrical Machine Preliminary Design with Artificial Intelligence Expert Guidance",
    None
);

witness_marker!(
    /// Practitioner guide for reliable production deployment of AI agent systems.
    DeliverAiAgentsPaper,
    "ai-llm/deliver-ai-agents-with-confidence",
    WitnessFamily::Paper,
    "Deliver AI Agents with Confidence",
    None
);

witness_marker!(
    /// Digital twin + ML for anomaly detection in power electronics dominated grids.
    DigitalTwinAnomalyPaper,
    "ai-llm/digital-twin-ml-anomaly-power-electronics",
    WitnessFamily::Paper,
    "Leveraging Digital Twin and Machine Learning Techniques for Anomaly Detection in Power Electronics Dominated Grid",
    None
);

witness_marker!(
    /// HEARSAY-II blackboard architecture (Erman et al. 1980) — PDF in AI_LLM/.
    ErmanHearsayIiAiLlmPaper,
    "ai-llm/erman-1980-hearsay-ii",
    WitnessFamily::Paper,
    "The HEARSAY-II Speech-Understanding System (Erman et al. 1980)",
    Some(1980)
);

witness_marker!(
    /// ETHOS framework: ethical technology and holistic oversight for AI agent systems.
    EthosAiAgentsPaper,
    "ai-llm/ethos-ai-agents-ethical-holistic-oversight",
    WitnessFamily::Paper,
    "On the ETHOS of AI Agents: An Ethical Technology and Holistic Oversight System",
    None
);

witness_marker!(
    /// Evolvable agent framework using event trees for emergency decision support.
    EventTreeEmergencyDecisionPaper,
    "ai-llm/evolvable-agents-event-trees-emergency-decision",
    WitnessFamily::Paper,
    "A Novel Task-Driven Method with Evolvable Interactive Agents Using Event Trees for Enhanced Emergency Decision Support",
    None
);

witness_marker!(
    /// Federated agent architecture for distributed scientific workflow orchestration.
    FederatedAgentWorkflowPaper,
    "ai-llm/federated-agents-scientific-workflows",
    WitnessFamily::Paper,
    "Empowering Scientific Workflows with Federated Agents",
    None
);

witness_marker!(
    /// DENDRAL expert system (Feigenbaum et al. 1971) — PDF in AI_LLM/.
    FeigenbaumDendralAiLlmPaper,
    "ai-llm/feigenbaum-1971-dendral",
    WitnessFamily::Paper,
    "The DENDRAL Project (Feigenbaum, Buchanan & Lederberg 1971)",
    Some(1971)
);

witness_marker!(
    /// STRIPS planner (Fikes & Nilsson 1971) — PDF in AI_LLM/, breed witness in cognition.
    FikesNilssonStripsAiLlmPaper,
    "ai-llm/fikes-nilsson-1971-strips",
    WitnessFamily::Paper,
    "STRIPS: A New Approach to the Application of Theorem Proving to Problem Solving (Fikes & Nilsson 1971)",
    Some(1971)
);

witness_marker!(
    /// G-Designer: GNN-based automatic design of multi-agent communication topologies.
    GDesignerMultiAgentPaper,
    "ai-llm/g-designer-multi-agent-gnn-topology",
    WitnessFamily::Paper,
    "G-Designer: Architecting Multi-agent Communication Topologies via Graph Neural Networks",
    None
);

witness_marker!(
    /// Generative AI impact on cross-border e-commerce brand building in manufacturing sector.
    GenAiEcommercePaper,
    "ai-llm/genai-cross-border-ecommerce-brand",
    WitnessFamily::Paper,
    "Exploring the Impact of Generative AI on Cross-Border E-Commerce Brand Building",
    None
);

witness_marker!(
    /// Geometry-grounded VLM with unified 3D reconstruction and spatial reasoning capabilities.
    GeometryVlmPaper,
    "ai-llm/vlm-geometry-grounded-3d-reasoning",
    WitnessFamily::Paper,
    "VLM: Geometry Grounded Vision Language Model with Unified 3D Reconstruction and Spatial Reasoning",
    None
);

witness_marker!(
    /// Graph of Thoughts: non-linear reasoning architecture for complex LLM problem solving.
    GraphOfThoughtsPaper,
    "ai-llm/graph-of-thoughts-elaborate-problem-solving",
    WitnessFamily::Paper,
    "Graph of Thoughts: Solving Elaborate Problems with Large Language Models",
    None
);

witness_marker!(
    /// HaVen: HDL-engineer-aligned LLM reducing hallucinations in Verilog code generation.
    HaVenVerilogPaper,
    "ai-llm/haven-llm-verilog-hallucination-mitigation",
    WitnessFamily::Paper,
    "HaVen: Hallucination-Mitigated LLM for Verilog Code Generation Aligned with HDL Engineers",
    None
);

witness_marker!(
    /// Comprehensive survey of hardware acceleration architectures for neural network inference.
    HardwareAccelerationNnPaper,
    "ai-llm/hardware-acceleration-neural-networks-survey",
    WitnessFamily::Paper,
    "Hardware Acceleration for Neural Networks: A Comprehensive Survey",
    None
);

witness_marker!(
    /// Theory and computation for n-person games with partial knowledge: human vs. machine.
    HumanMachineNPersonGamesPaper,
    "ai-llm/human-machine-n-person-games-partial-knowledge",
    WitnessFamily::Paper,
    "Human and Machine Intelligence in n-Person Games with Partial Knowledge: Theory and Computation",
    None
);

witness_marker!(
    /// Hybrid cloud architecture transformation strategies for emerging AI workload demands.
    HybridCloudAiPaper,
    "ai-llm/hybrid-cloud-emerging-ai-workloads",
    WitnessFamily::Paper,
    "Transforming the Hybrid Cloud for Emerging AI Workloads",
    None
);

witness_marker!(
    /// InPars+: enhanced synthetic data generation for information retrieval benchmarking.
    InParsPlusPaper,
    "ai-llm/inpars-plus-synthetic-data-ir",
    WitnessFamily::Paper,
    "InPars+: Supercharging Synthetic Data Generation for Information Retrieval Systems",
    None
);

witness_marker!(
    /// Prolog / predicate logic as programming language (Kowalski 1974) — PDF in AI_LLM/.
    KowalskiPrologAiLlmPaper,
    "ai-llm/kowalski-1974-predicate-logic-programming",
    WitnessFamily::Paper,
    "Predicate Logic as Programming Language (Kowalski 1974)",
    Some(1974)
);

witness_marker!(
    /// SOAR cognitive architecture (Laird et al. 1987) — PDF in AI_LLM/.
    LairdSoarAiLlmPaper,
    "ai-llm/laird-1987-soar",
    WitnessFamily::Paper,
    "SOAR: An Architecture for General Intelligence (Laird, Newell & Rosenbloom 1987)",
    Some(1987)
);

witness_marker!(
    /// LaMMA-P: LLM-driven PDDL planner for multi-agent long-horizon task allocation.
    LammaPPaper,
    "ai-llm/lamma-p-multi-agent-pddl-long-horizon",
    WitnessFamily::Paper,
    "LaMMA-P: Generalizable Multi-Agent Long-Horizon Task Allocation and Planning with LM-Driven PDDL Planner",
    None
);

witness_marker!(
    /// Reading order as ordering relations for structured visually-rich document understanding.
    LayoutReadingOrderPaper,
    "ai-llm/layout-reading-order-ordering-relations-vrd",
    WitnessFamily::Paper,
    "Modeling Layout Reading Order as Ordering Relations for Visually-rich Document Understanding",
    None
);

witness_marker!(
    /// RL agent learning to navigate web interfaces from raw DOM and visual observations.
    LearningNavigateWebPaper,
    "ai-llm/learning-navigate-web-rl-agent",
    WitnessFamily::Paper,
    "Learning to Navigate the Web",
    None
);

witness_marker!(
    /// Agentic RL for iterative SPARQL query refinement in KG question answering.
    LearningRefineSparqlPaper,
    "ai-llm/learning-refine-agentic-rl-sparql",
    WitnessFamily::Paper,
    "Learning to Refine: An Agentic RL Approach for Iterative SPARQL Query Construction",
    None
);

witness_marker!(
    /// Evaluation framework for hallucinations in LLM-powered code generation systems.
    LlmCodeHallucinationsPaper,
    "ai-llm/llm-code-hallucinations-evaluation",
    WitnessFamily::Paper,
    "Exploring and Evaluating Hallucinations in LLM-Powered Code Generation",
    None
);

witness_marker!(
    /// LLM-based comprehensibility assessment of complex multi-structured financial documents.
    LlmFinancialDocumentsPaper,
    "ai-llm/llm-financial-documents-comprehensibility",
    WitnessFamily::Paper,
    "On the Comprehensibility of Multi-structured Financial Documents using LLMs and Pre-processing Tools",
    None
);

witness_marker!(
    /// LLM-First Search: self-guided solution space exploration using LLM heuristics.
    LlmFirstSearchPaper,
    "ai-llm/llm-first-search-self-guided",
    WitnessFamily::Paper,
    "LLM-First Search: Self-Guided Exploration of the Solution Space",
    None
);

witness_marker!(
    /// LLM-guided scenario generation and execution for GUI application testing.
    LlmGuiTestingPaper,
    "ai-llm/llm-scenario-gui-testing",
    WitnessFamily::Paper,
    "LLM-Guided Scenario-based GUI Testing",
    None
);

witness_marker!(
    /// Empirical study of GitHub Copilot's causal impact on open-source innovation metrics.
    LlmOpenSourceInnovationPaper,
    "ai-llm/llm-open-source-innovation-github-copilot",
    WitnessFamily::Paper,
    "The Impact of Large Language Models on Open-source Innovation: Evidence from GitHub Copilot",
    None
);

witness_marker!(
    /// Empirical study of issues, root causes, and fixes in LLM open-source repositories.
    LlmOpenSourceIssuesPaper,
    "ai-llm/llm-open-source-issues-causes-solutions",
    WitnessFamily::Paper,
    "Demystifying Issues, Causes and Solutions in LLM Open-Source Projects",
    None
);

witness_marker!(
    /// LLM reasoning and planning for handling incomplete user queries over APIs.
    LlmReasoningPlanningPaper,
    "ai-llm/llm-reasoning-planning-incomplete-api-queries",
    WitnessFamily::Paper,
    "LLM+Reasoning+Planning for Supporting Incomplete User Queries in the Presence of APIs",
    None
);

witness_marker!(
    /// Evaluation of LLMs for automated security policy authoring and enforcement.
    LlmSecurityPoliciesPaper,
    "ai-llm/llm-automating-security-policies",
    WitnessFamily::Paper,
    "On Automating Security Policies with Contemporary LLMs",
    None
);

witness_marker!(
    /// LLM-assisted design of semantic web multi-agent systems using AJAN framework.
    LlmSemanticWebAjanPaper,
    "ai-llm/llm-semantic-web-multi-agent-ajan",
    WitnessFamily::Paper,
    "LLM-Assisted Modeling of Semantic Web-Enabled Multi-Agent Systems with AJAN",
    None
);

witness_marker!(
    /// Reconceptualizing LLMs through structuralist and post-structuralist linguistic theory.
    LlmSemioticMachinesPaper,
    "ai-llm/llm-semiotic-machines-structuralist",
    WitnessFamily::Paper,
    "Language Models as Semiotic Machines: Reconceptualizing AI Language Systems through Structuralist and Post-Structuralist Theories of Language",
    None
);

witness_marker!(
    /// LLM framework for context-aware automated test code refactoring.
    LlmTestRefactoringPaper,
    "ai-llm/llm-automatic-test-refactoring",
    WitnessFamily::Paper,
    "Context-Enhanced LLM-Based Framework for Automatic Test Refactoring",
    None
);

witness_marker!(
    /// ML impact on engineering: analysis paralysis, infeasible optima, and Rashomon paradox.
    MachineLearningAnalysisParadoxPaper,
    "ai-llm/ml-analysis-paralysis-rashomon-paradox",
    WitnessFamily::Paper,
    "A Look into How Machine Learning is Reshaping Engineering Models: the Rise of Analysis Paralysis, Optimal yet Infeasible Solutions, and the Inevitable Rashomon Paradox",
    None
);

witness_marker!(
    /// MACI: multi-agent collaborative intelligence for robust reasoning and temporal planning.
    MaciMultiAgentPaper,
    "ai-llm/maci-multi-agent-collaborative-intelligence",
    WitnessFamily::Paper,
    "MACI: Multi-Agent Collaborative Intelligence for Robust Reasoning and Temporal Planning",
    None
);

witness_marker!(
    /// LLM with context-aware prompting for manufacturing knowledge base access.
    ManufacturingKnowledgeLlmPaper,
    "ai-llm/manufacturing-knowledge-llm-context-prompting",
    WitnessFamily::Paper,
    "Enhancing Manufacturing Knowledge Access with LLMs and Context-aware Prompting",
    None
);

witness_marker!(
    /// Marr–Poggio stereo disparity (1976) — PDF in AI_LLM/.
    MarrPoggioVisionAiLlmPaper,
    "ai-llm/marr-poggio-1976-stereo-disparity",
    WitnessFamily::Paper,
    "Cooperative Computation of Stereo Disparity (Marr & Poggio 1976)",
    Some(1976)
);

witness_marker!(
    /// Multimodal LLM workflow decomposition for automated mobile UI annotation.
    MaydayMultimodalPaper,
    "ai-llm/context-aware-workflow-mobile-ui-annotation",
    WitnessFamily::Paper,
    "Context-Aware Workflow Decomposition for Automated Mobile UI Annotation Using Multimodal Large Language Models",
    None
);

witness_marker!(
    /// Meta-Prompting Protocol: adversarial feedback loops for orchestrating LLM ensembles.
    MetaPromptingProtocolPaper,
    "ai-llm/meta-prompting-protocol-adversarial-feedback",
    WitnessFamily::Paper,
    "The Meta-Prompting Protocol: Orchestrating LLMs via Adversarial Feedback Loops",
    None
);

witness_marker!(
    /// Survey of ML/DL techniques applied to cybersecurity and digital forensics domains.
    MlCybersecurityReviewPaper,
    "ai-llm/ml-cybersecurity-digital-forensics-review",
    WitnessFamily::Paper,
    "Machine Learning and Deep Learning Techniques used in Cybersecurity and Digital Forensics: a Review",
    None
);

witness_marker!(
    /// MLE-bench: benchmark evaluating AI agents on real machine learning engineering tasks.
    MleBenchPaper,
    "ai-llm/mle-bench-ml-agents-engineering-evaluation",
    WitnessFamily::Paper,
    "MLE-bench: Evaluating Machine Learning Agents on Machine Learning Engineering",
    None
);

witness_marker!(
    /// Neuro-symbolic LM for fast accurate task planning via multi-level goal decomposition.
    NeuroSymbolicTaskPlanningPaper,
    "ai-llm/neuro-symbolic-task-planning-goal-decomposition",
    WitnessFamily::Paper,
    "Fast and Accurate Task Planning using Neuro-Symbolic Language Models and Multi-level Goal Decomposition",
    None
);

witness_marker!(
    /// General Problem Solver (Newell & Simon 1963) — PDF in AI_LLM/.
    NewellSimonGpsAiLlmPaper,
    "ai-llm/newell-simon-1963-gps",
    WitnessFamily::Paper,
    "GPS: A Program that Simulates Human Thought (Newell & Simon 1963)",
    Some(1963)
);

witness_marker!(
    /// OCEL as process intelligence foundation for generative, predictive, and prescriptive AI.
    NoAiWithoutPiPaper,
    "ai-llm/no-ai-without-pi-ocel-generative-predictive",
    WitnessFamily::Paper,
    "No AI Without PI! Object-Centric Process Mining as the Enabler for Generative, Predictive, and Prescriptive AI",
    None
);

witness_marker!(
    /// Optimal PDDL/POPF-based task planning and agent-aware allocation for collaborative tasks.
    OptimalTaskPlanningPddlPaper,
    "ai-llm/optimal-task-planning-pddl-popf",
    WitnessFamily::Paper,
    "An Optimal Task Planning and Agent-aware Allocation Algorithm in Collaborative Tasks Combining with PDDL and POPF",
    None
);

witness_marker!(
    /// ORTAC+: domain-specific language for user-friendly multi-agent mission planning.
    OrtacPlusMultiAgentPaper,
    "ai-llm/ortac-plus-dsl-multi-agent-mission",
    WitnessFamily::Paper,
    "ORTAC+: A User Friendly Domain Specific Language for Multi-Agent Mission Planning",
    None
);

witness_marker!(
    /// Environment interaction for automated PDDL domain/problem generation with LLMs.
    PddlEnvInteractionPaper,
    "ai-llm/pddl-env-interaction-llm-planning",
    WitnessFamily::Paper,
    "Leveraging Environment Interaction for Automated PDDL Generation and Planning with Large Language Models",
    None
);

witness_marker!(
    /// Model-based workflow for automated PDDL domain and problem description generation.
    PddlWorkflowGenerationPaper,
    "ai-llm/pddl-workflow-automated-generation",
    WitnessFamily::Paper,
    "Model-based Workflow for the Automated Generation of PDDL Descriptions",
    None
);

witness_marker!(
    /// Pix2Struct: screenshot-parsing pretraining for visual language model understanding.
    Pix2StructPaper,
    "ai-llm/pix2struct-screenshot-parsing-pretraining",
    WitnessFamily::Paper,
    "Pix2Struct: Screenshot Parsing as Pretraining for Visual Language Understanding",
    None
);

witness_marker!(
    /// Proxy task framework for predicting LLM emergent ability phase transitions.
    PredictableEmergentAbilitiesPaper,
    "ai-llm/predictable-emergent-abilities-llm-proxy",
    WitnessFamily::Paper,
    "Predictable Emergent Abilities of LLMs: Proxy Tasks Are All You Need",
    None
);

witness_marker!(
    /// Framing LLM prompt optimization as a state-space search problem with heuristic guidance.
    PromptOptimizationPaper,
    "ai-llm/prompt-optimization-state-space-search",
    WitnessFamily::Paper,
    "Prompt Optimization as a State-Space Search Problem",
    None
);

witness_marker!(
    /// LLM-based intelligent failure management system for public infrastructure.
    PublicFacilityLlmPaper,
    "ai-llm/public-facility-failure-management-llm",
    WitnessFamily::Paper,
    "Sustainable and Intelligent Public Facility Failure Management System Based on Large Language Models",
    None
);

witness_marker!(
    /// QiMeng: end-to-end automated hardware and software co-design for processor chips.
    QiMengChipDesignPaper,
    "ai-llm/qimeng-automated-hw-sw-processor-chip",
    WitnessFamily::Paper,
    "QiMeng: Fully Automated Hardware and Software Design for Processor Chip",
    None
);

witness_marker!(
    /// RePrompt: automated prompt engineering for planning-capable LLM agents.
    RePromptPaper,
    "ai-llm/reprompt-automatic-prompt-engineering-agents",
    WitnessFamily::Paper,
    "RePrompt: Planning by Automatic Prompt Engineering for Large Language Models Agents",
    None
);

witness_marker!(
    /// Interaction-of-Thought reasoning for explainable LLM-based recommendation systems.
    ReasonToRecommendPaper,
    "ai-llm/reason-to-recommend-iot-reasoning",
    WitnessFamily::Paper,
    "Reason-to-Recommend: Using Interaction-of-Thought Reasoning to Enhance LLM Recommendation",
    None
);

witness_marker!(
    /// Robinson resolution principle (1965) — PDF in AI_LLM/.
    RobinsonResolutionAiLlmPaper,
    "ai-llm/robinson-1965-resolution-principle",
    WitnessFamily::Paper,
    "A Machine-Oriented Logic Based on the Resolution Principle (Robinson 1965)",
    Some(1965)
);

witness_marker!(
    /// RPG: repository planning graph enabling unified scalable codebase generation.
    RpgCodebaseGenerationPaper,
    "ai-llm/rpg-repository-planning-graph-codebase-gen",
    WitnessFamily::Paper,
    "RPG: A Repository Planning Graph for Unified and Scalable Codebase Generation",
    None
);

witness_marker!(
    /// Search-on-Graph: iterative informed KG navigation for LLM reasoning tasks.
    SearchOnGraphPaper,
    "ai-llm/search-on-graph-llm-reasoning-kg",
    WitnessFamily::Paper,
    "Search-on-Graph: Iterative Informed Navigation for Large Language Model Reasoning on Knowledge Graphs",
    None
);

witness_marker!(
    /// Goal-oriented semantic communication for robot arm digital twin reconstruction.
    SemanticCommunicationRobotPaper,
    "ai-llm/semantic-communication-robot-arm-digital-twin",
    WitnessFamily::Paper,
    "Goal-oriented Semantic Communication for Robot Arm Reconstruction in Digital Twin",
    None
);

witness_marker!(
    /// Semantic constraint inference from web forms for automated test generation.
    SemanticConstraintWebFormPaper,
    "ai-llm/semantic-constraint-web-form-test-gen",
    WitnessFamily::Paper,
    "Semantic Constraint Inference for Web Form Test Generation",
    None
);

witness_marker!(
    /// MYCIN certainty factor model (Shortliffe & Buchanan 1975) — PDF in AI_LLM/.
    ShortliffeMycinAiLlmPaper,
    "ai-llm/shortliffe-1975-mycin-cf-model",
    WitnessFamily::Paper,
    "A Model of Inexact Reasoning in Medicine (Shortliffe & Buchanan 1975)",
    Some(1975)
);

witness_marker!(
    /// SMoG: graph-based schema matching for heterogeneous data source integration.
    SmoGSchemaMatchingPaper,
    "ai-llm/smog-schema-matching-graph",
    WitnessFamily::Paper,
    "SMoG: Schema Matching on Graph",
    None
);

witness_marker!(
    /// Step-by-step training curriculum for enhancing LLM soft constraint adherence.
    SoftConstraintLlmPaper,
    "ai-llm/soft-constraint-following-llm-step-by-step",
    WitnessFamily::Paper,
    "Step-by-Step Mastery: Enhancing Soft Constraint Following Ability of Large Language Models",
    None
);

witness_marker!(
    /// Solutions Architect's Handbook: AI/cloud architecture patterns and practices.
    SolutionsArchitectsHandbookPaper,
    "ai-llm/solutions-architects-handbook",
    WitnessFamily::Paper,
    "Solutions Architect's Handbook",
    None
);

witness_marker!(
    /// HACKER learning by debugging (Sussman 1973) — PDF in AI_LLM/.
    SussmanHackerAiLlmPaper,
    "ai-llm/sussman-1973-hacker",
    WitnessFamily::Paper,
    "A Computational Model of Skill Acquisition (Sussman 1973 — HACKER)",
    Some(1973)
);

witness_marker!(
    /// European Parliament document TA-9-2024-0138 — regulatory text (likely AI Act related).
    Ta9RegulationPaper,
    "ai-llm/ta-9-2024-0138-eu-regulation",
    WitnessFamily::Standard,
    "TA-9-2024-0138_EN: EU Regulatory Document",
    None
);

witness_marker!(
    /// ThriftLLM: cost-effective routing of classification queries to appropriately-sized LLMs.
    ThriftLlmPaper,
    "ai-llm/thriftllm-cost-effective-llm-selection",
    WitnessFamily::Paper,
    "ThriftLLM: On Cost-Effective Selection of Large Language Models for Classification Queries",
    None
);

witness_marker!(
    /// RAG-based mathematical reasoning for UAV mission planning and arithmetic tasks.
    UavMathRagPaper,
    "ai-llm/uav-math-reasoning-rag",
    WitnessFamily::Paper,
    "Mathematical Reasoning for Unmanned Aerial Vehicles: A RAG-Based Approach for Complex Arithmetic Reasoning",
    None
);

witness_marker!(
    /// Universal PDDL domain specification for cross-domain task planning benchmarking.
    UniversalPddlDomainPaper,
    "ai-llm/universal-pddl-domain",
    WitnessFamily::Paper,
    "The Universal PDDL Domain",
    None
);
