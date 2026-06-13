












//! Business/Security/Maturity/BEAM/General witness markers — compiled from `ontology/witnesses-domain.ttl`.
//!
//! Every entry here is derived from a `compat:WitnessMarker` tagged
//! `compat:paperCategory "domain"` in the ontology.
//!
//! To add a paper: declare a `compat:WitnessMarker` with
//! `compat:paperCategory "domain"` in
//! `ontology/witnesses-domain.ttl`, then run
//! `cargo make ggen-witnesses-domain`.
//!
//! Structure-only authority labels. Graduate to `wasm4pm` when the
//! paper's algorithm must actually execute.


use crate::witness::WitnessFamily;


witness_marker!(
    /// Psychological antecedents to autonomous self-organization in Agile Scrum teams.
    AgileTeamAutonomyPaper,
    "domain/agile-scrum-team-autonomy-antecedents",
    WitnessFamily::Paper,
    "Psychological Antecedents to Emergence of Team Autonomy in Agile Scrum Teams",
    None
);

witness_marker!(
    /// Integrated crop model, cloud, and big data analytics for agriculture activity monitoring.
    AgricultureCloudBigDataPaper,
    "domain/agriculture-cloud-bigdata-activity-monitoring",
    WitnessFamily::Paper,
    "An Integrated (Crop Model, Cloud and Big Data Analytic) Framework to Support Agriculture Activity Monitoring System",
    None
);

witness_marker!(
    /// OMG AI standards portfolio — normative specifications for AI interoperability.
    AiOmgStandardsPaper,
    "domain/artificial-intelligence-omg-standards",
    WitnessFamily::Standard,
    "Artificial Intelligence OMG Standards",
    None
);

witness_marker!(
    /// NIST AI RMF-grounded maturity model for AI risk management capability.
    AiRiskMaturityNistPaper,
    "domain/ai-risk-management-maturity-nist-rmf",
    WitnessFamily::Paper,
    "Evolving AI Risk Management: A Maturity Model based on the NIST AI Risk Management Framework",
    None
);

witness_marker!(
    /// AI applications in ship finance with case study: AI-augmented loan origination process.
    AiShipFinancePaper,
    "domain/ai-ship-finance-loan-origination",
    WitnessFamily::Paper,
    "Artificial Intelligence in Ship Finance: Applications, Opportunities, and a Case Study in AI-Augmented Loan Origination",
    None
);

witness_marker!(
    /// Ansible Workshop: hands-on infrastructure automation with Ansible playbooks.
    AnsibleWorkshopPaper,
    "domain/ansible-workshop-automation",
    WitnessFamily::Paper,
    "The Ansible Workshop",
    None
);

witness_marker!(
    /// Source data and evidence base for the API management focus area maturity model.
    ApiManagementMaturitySourcePaper,
    "domain/api-management-maturity-focus-area",
    WitnessFamily::Paper,
    "Source Data for the Focus Area Maturity Model for API Management",
    None
);

witness_marker!(
    /// Ash Framework B5.0: resource-oriented application framework for Elixir.
    AshFrameworkElixirPaper,
    "domain/ash-framework-elixir-b5",
    WitnessFamily::Paper,
    "Ash Framework for Elixir (B5.0)",
    None
);

witness_marker!(
    /// ATLAS: multi-agent LLM system for adaptive trading via dynamic prompt optimization.
    AtlasAdaptiveTradingPaper,
    "domain/atlas-adaptive-trading-llm-multi-agent",
    WitnessFamily::Paper,
    "ATLAS: Adaptive Trading with LLM AgentS Through Dynamic Prompt Optimization and Multi-Agent Coordination",
    None
);

witness_marker!(
    /// Maturity model for assessing organizational audit function capability.
    AuditMaturityModelPaper,
    "domain/audit-maturity-model",
    WitnessFamily::Paper,
    "Audit Maturity Model",
    None
);

witness_marker!(
    /// Analysis document for auto and property insurance domain processes.
    AutoPropertyInsurancePaper,
    "domain/auto-property-insurance-analysis",
    WitnessFamily::Paper,
    "Auto-Property Insurance Analysis",
    None
);

witness_marker!(
    /// Auto-Tables: automatic multi-step transformation synthesis to relationalize messy tables.
    AutoTablesRelationalizePaper,
    "domain/auto-tables-multi-step-transformations",
    WitnessFamily::Paper,
    "Auto-Tables: Synthesizing Multi-Step Transformations to Relationalize Tables without Using Examples",
    None
);

witness_marker!(
    /// Zero-sum trust dynamics: automation as a threat to human epistemic agency.
    AutomationTrustEpistemicPaper,
    "domain/automation-trust-zero-sum-epistemic-agency",
    WitnessFamily::Paper,
    "When Trust is Zero Sum: Automation Threat to Epistemic Agency",
    None
);

witness_marker!(
    /// BARTPredict: LLM-driven cyber threat prediction for IoT security environments.
    BartPredictIotSecurityPaper,
    "domain/bartpredict-llm-iot-cyber-threat",
    WitnessFamily::Paper,
    "BARTPredict: Empowering IoT Security with LLM-Driven Cyber Threat Prediction",
    None
);

witness_marker!(
    /// Bayesian network model linking PM maturity level to project overcost risk.
    BayesianNetworkProjectRiskPaper,
    "domain/bayesian-network-project-maturity-risk",
    WitnessFamily::Paper,
    "Use of Bayesian Network Characteristics to Link Project Management Maturity and Risk of Project Overcost",
    None
);

witness_marker!(
    /// BLAKE3 cryptographic hash function (O'Connor et al. 2020): design, security, and performance.
    Blake3HashFunctionPaper,
    "domain/blake3-hash-function-2020",
    WitnessFamily::Paper,
    "BLAKE3: One Function, Fast Everywhere (arXiv:2012.04616)",
    Some(2020)
);

witness_marker!(
    /// Quantitative blockchain trilemma analysis across Algorand, Ethereum 2.0, and others.
    BlockchainTrilemmaAlgorandPaper,
    "domain/blockchain-trilemma-algorand-ethereum-analysis",
    WitnessFamily::Paper,
    "Quantifying the Blockchain Trilemma: A Comparative Analysis of Algorand, Ethereum 2.0, and Beyond",
    None
);

witness_marker!(
    /// Blockchain and smart contract architecture for secure vehicle auction processes.
    BlockchainVehicleAuctionPaper,
    "domain/blockchain-secure-vehicle-auction-smart-contracts",
    WitnessFamily::Paper,
    "Blockchain-Based Secure Vehicle Auction System with Smart Contracts",
    None
);

witness_marker!(
    /// BPR in supply chains applied to the expanding Halal industry case study.
    BprSupplyChainHalalPaper,
    "domain/bpr-supply-chain-halal-industry",
    WitnessFamily::Paper,
    "Business Process Re-engineering in Supply Chains: Examining the Case of the Expanding Halal Industry",
    None
);

witness_marker!(
    /// Broadway: multi-stage data processing pipeline library for Elixir.
    BroadwayElixirPaper,
    "domain/broadway-elixir-data-processing",
    WitnessFamily::Paper,
    "Broadway: Data Processing Pipelines for Elixir",
    None
);

witness_marker!(
    /// C4ISR architecture framework applied to simulation system VV&A in military domains.
    C4isrVvaPaper,
    "domain/c4isr-architecture-framework-vva-simulation",
    WitnessFamily::Paper,
    "Using the C4ISR Architecture Framework as a Tool to Facilitate VV&A for Simulation Systems within the Military Application Domain",
    None
);

witness_marker!(
    /// Category-theoretic foundations for probability theory and stochastic processes.
    CategoryTheoryProbabilityPaper,
    "domain/category-theory-probability-foundations",
    WitnessFamily::Paper,
    "Category Theory: Probability Foundations",
    None
);

witness_marker!(
    /// SoK: systematic survey of CCMMs — limitations, gaps, and bridging strategies.
    CcmmCybersecurityCapabilityPaper,
    "domain/ccmm-cybersecurity-capability-maturity-limitations",
    WitnessFamily::Paper,
    "SoK: Identifying Limitations and Bridging Gaps of Cybersecurity Capability Maturity Models (CCMMs)",
    None
);

witness_marker!(
    /// CDN architectures, performance optimization strategies, and future trend analysis.
    CdnArchitecturePerformancePaper,
    "domain/cdn-architecture-performance-future-trends",
    WitnessFamily::Paper,
    "Optimizing Digital Experiences with Content Delivery Networks: Architectures, Performance Strategies, and Future Trends",
    None
);

witness_marker!(
    /// Blueprint document for establishing and operating a Center of Excellence.
    CenterOfExcellencePaper,
    "domain/center-of-excellence-blueprint",
    WitnessFamily::Paper,
    "Center of Excellence: Blueprint for Cyberinfrastructure",
    None
);

witness_marker!(
    /// Closed-form task success probability expressions for status-driven BEAM systems.
    ClosedFormTaskSuccessBeamPaper,
    "domain/closed-form-task-success-status-driven",
    WitnessFamily::Paper,
    "Closed-Form and Boundary Expressions for Task-Success Probability in Status-Driven Systems",
    None
);

witness_marker!(
    /// CMMI + ITIL integrated SQA service maturity model for software quality assurance.
    CmmiItilSqaMaturityPaper,
    "domain/cmmi-itil-sqa-service-maturity",
    WitnessFamily::Paper,
    "Innovative SQA Service Maturity Model using CMMI and ITIL",
    None
);

witness_marker!(
    /// Practitioner guide for managing complex multi-workstream programs.
    ComplexProgramManagementPaper,
    "domain/complex-program-management",
    WitnessFamily::Paper,
    "How to Manage Complex Programs",
    None
);

witness_marker!(
    /// Formal model grounding autonomous agency in computational irreducibility and undecidability.
    ComputationalIrreducibilityAgencyPaper,
    "domain/computational-irreducibility-agency-undecidability",
    WitnessFamily::Paper,
    "Computational Irreducibility as the Foundation of Agency: A Formal Model Connecting Undecidability to Autonomous Behavior in Complex Systems",
    None
);

witness_marker!(
    /// Guide to concurrent data processing patterns using Elixir GenStage and Flow.
    ConcurrentDataProcessingElixirPaper,
    "domain/concurrent-data-processing-elixir",
    WitnessFamily::Paper,
    "Concurrent Data Processing in Elixir",
    None
);

witness_marker!(
    /// CoRL environment creation and management framework with system integration focus.
    CorlEnvManagementPaper,
    "domain/corl-environment-system-integration",
    WitnessFamily::Paper,
    "CoRL: Environment Creation and Management Focused on System Integration",
    None
);

witness_marker!(
    /// Maturity model for organizational crypto-agility: readiness for cryptographic algorithm migration.
    CryptoAgilityMaturityPaper,
    "domain/crypto-agility-maturity-assessment",
    WitnessFamily::Paper,
    "Towards a Maturity Model for Crypto-Agility Assessment",
    None
);

witness_marker!(
    /// CMMI + XP defect prevention integration for software process improvement.
    DefectPreventionCmmiXpPaper,
    "domain/defect-prevention-cmmi-xp-integration",
    WitnessFamily::Paper,
    "Software Process Improvement Based on Defect Prevention Using Capability and Testing Model Integration in Extreme Programming",
    None
);

witness_marker!(
    /// NIST/government guidelines on minimum software developer verification standards.
    DeveloperVerificationGuidelinesPaper,
    "domain/developer-verification-software-guidelines",
    WitnessFamily::Standard,
    "Guidelines on Minimum Standards for Developer Verification of Software",
    None
);

witness_marker!(
    /// Digital business ecosystem maturity model tailored to personal service firm contexts.
    DigitalBusinessEcosystemMaturityPaper,
    "domain/digital-business-ecosystem-maturity-service-firms",
    WitnessFamily::Paper,
    "A Digital Business Ecosystem Maturity Model for Personal Service Firms",
    None
);

witness_marker!(
    /// DGMM: maturity model for digital game development and release processes.
    DigitalGameMaturityPaper,
    "domain/digital-game-maturity-model",
    WitnessFamily::Paper,
    "A Digital Game Maturity Model (DGMM)",
    None
);

witness_marker!(
    /// Tutorial and survey on digital twin technologies for emerging cellular network management.
    DigitalTwinCellularNetworksPaper,
    "domain/digital-twin-emerging-cellular-networks-survey",
    WitnessFamily::Paper,
    "From Simulators to Digital Twins for Enabling Emerging Cellular Networks: A Tutorial and Survey",
    None
);

witness_marker!(
    /// Maturity model construction methodology for geographically distributed software organizations.
    DistributedSoftwareMaturityPaper,
    "domain/distributed-software-org-maturity-model",
    WitnessFamily::Paper,
    "Constructing a Maturity Model for a Distributed Software Organization",
    None
);

witness_marker!(
    /// Engineering Elixir Applications: OTP patterns, deployment, and production practices.
    EngineeringElixirApplicationsPaper,
    "domain/engineering-elixir-applications",
    WitnessFamily::Paper,
    "Engineering Elixir Applications",
    None
);

witness_marker!(
    /// Mathematical study of excess growth rate dynamics in portfolio theory.
    ExcessGrowthRatePaper,
    "domain/excess-growth-rate-mathematical-study",
    WitnessFamily::Paper,
    "A Mathematical Study of the Excess Growth Rate",
    None
);

witness_marker!(
    /// FAPL-DM-BC: federated learning with adaptive privacy, dynamic masking, blockchain for IoV.
    FaplFederatedLearningIovPaper,
    "domain/fapl-federated-learning-privacy-iov",
    WitnessFamily::Paper,
    "FAPL-DM-BC: A Secure and Scalable FL Framework with Adaptive Privacy and Dynamic Masking, Blockchain, and XAI for the IoVs",
    None
);

witness_marker!(
    /// Observability patterns and tools for distributed fog computing architectures.
    FogComputingObservabilityPaper,
    "domain/observability-fog-computing",
    WitnessFamily::Paper,
    "Observability in Fog Computing",
    None
);

witness_marker!(
    /// Genetic algorithm implementation and evolution strategies using Elixir and Nx.
    GeneticAlgorithmsElixirPaper,
    "domain/genetic-algorithms-elixir",
    WitnessFamily::Paper,
    "Genetic Algorithms in Elixir",
    None
);

witness_marker!(
    /// Geno: developer tool for adding multimodal interaction layers to existing web apps.
    GenoMultimodalWebPaper,
    "domain/geno-multimodal-interaction-web-applications",
    WitnessFamily::Paper,
    "Geno: A Developer Tool for Authoring Multimodal Interaction on Existing Web Applications",
    None
);

witness_marker!(
    /// Change management and traceability framework for dependable global software development.
    GlobalSoftwareChangeManagementPaper,
    "domain/change-management-traceability-global-software",
    WitnessFamily::Paper,
    "Towards Dependable Change Management and Traceability for Global Software Development",
    None
);

witness_marker!(
    /// Farnam Street: general thinking tools and mental models for better decision-making.
    GreatMentalModelsPaper,
    "domain/great-mental-models-farnam-street",
    WitnessFamily::Paper,
    "The Great Mental Models (Farnam Street)",
    None
);

witness_marker!(
    /// Frustration index hierarchy applied to U.S. intermarket complex system dynamics.
    HierarchyFrustrationsIntermarketPaper,
    "domain/hierarchy-frustrations-complex-intermarket",
    WitnessFamily::Paper,
    "Hierarchy of Frustrations as Supplementary Indices in Complex System Dynamics, Applied to the U.S. Intermarket",
    None
);

witness_marker!(
    /// Study of HFT strategies and their relationship to mini flash crash events.
    HighFrequencyTradingMiniFlashPaper,
    "domain/hft-mini-flash-crashes",
    WitnessFamily::Paper,
    "High Frequency Trading and Mini Flash Crashes",
    None
);

witness_marker!(
    /// ICS-CTM2: maturity model for industrial control system cybersecurity testbeds.
    IcsCtm2CybersecurityMaturityPaper,
    "domain/ics-ctm2-industrial-cybersecurity-maturity",
    WitnessFamily::Paper,
    "ICS-CTM2: Industrial Control System Cybersecurity Testbed Maturity Model",
    None
);

witness_marker!(
    /// Information governance framework applied to evaluate patient care quality at One Medical.
    InformationGovernanceAmazonOneMedicalPaper,
    "domain/information-governance-amazon-one-medical",
    WitnessFamily::Paper,
    "Using Information Governance to Evaluate Patient Care in Amazon's One Medical",
    None
);

witness_marker!(
    /// instructor_ex quickstart: structured LLM output library for Elixir applications.
    InstructorExElixirPaper,
    "domain/instructor-ex-elixir-structured-output",
    WitnessFamily::Paper,
    "instructor_ex: Structured Outputs for Elixir LLM Applications",
    None
);

witness_marker!(
    /// ML evaluation of the EMH using intermarket data correlations across asset classes.
    IntermarketEfficientMarketHypothesisPaper,
    "domain/intermarket-emh-ml-evaluation",
    WitnessFamily::Paper,
    "Using Intermarket Data to Evaluate the Efficient Market Hypothesis with Machine Learning",
    None
);

witness_marker!(
    /// Katsanos: quantitative intermarket trading strategies for equities, bonds, and commodities.
    IntermarketTradingStrategiesPaper,
    "domain/intermarket-trading-strategies-katsanos",
    WitnessFamily::Paper,
    "Intermarket Trading Strategies (Katsanos)",
    None
);

witness_marker!(
    /// Introductory guide for teams adopting Elixir: philosophy, tooling, and migration patterns.
    IntroAdoptingElixirPaper,
    "domain/intro-adopting-elixir",
    WitnessFamily::Paper,
    "Intro to Adopting Elixir",
    None
);

witness_marker!(
    /// Survival analysis model predicting vulnerability fix times for IoT devices.
    IotVulnerabilityFixTimesPaper,
    "domain/iot-vulnerability-fix-times-survival-model",
    WitnessFamily::Paper,
    "Predicting IoT Device Vulnerability Fix Times with Survival and Failure Time Models",
    None
);

witness_marker!(
    /// LLM-based ISIC business activity classification framework for circular economy analysis.
    IsicClassificationLlmPaper,
    "domain/isic-classification-llm-circular-economy",
    WitnessFamily::Paper,
    "A Unified Framework to Classify Business Activities into ISIC through Large Language Models for Circular Economy",
    None
);

witness_marker!(
    /// ISM3 enhancement addressing human factors in organizational information sharing security.
    Ism3HumanFactorsPaper,
    "domain/ism3-human-factors-security-maturity",
    WitnessFamily::Paper,
    "Enhancing the Conventional Information Security Management Maturity Model (ISM3) in Resolving Human Factors in Organization Information Sharing",
    None
);

witness_marker!(
    /// Stop-and-reverse MinMax process for measuring lead-lag relationships in financial markets.
    LeadLagStopReversePaper,
    "domain/lead-lag-stop-reverse-minmax",
    WitnessFamily::Paper,
    "Lead-Lag Relationship using a Stop-and-Reverse MinMax Process",
    None
);

witness_marker!(
    /// SAP LeanIX full business capability map for enterprise architecture modeling.
    LeanIxCapabilityMapFullPaper,
    "domain/sap-leanix-business-capability-map-full",
    WitnessFamily::Paper,
    "SAP LeanIX Business Capability Map (Full)",
    None
);

witness_marker!(
    /// SAP LeanIX lightweight business capability map for quick EA reference.
    LeanIxCapabilityMapLightPaper,
    "domain/sap-leanix-business-capability-map-light",
    WitnessFamily::Paper,
    "SAP LeanIX Business Capability Map (Light)",
    None
);

witness_marker!(
    /// Lifestyle Redesign OT intervention: optimizing occupations and habits for health and well-being.
    LifestyleRedesignOccupationalPaper,
    "domain/lifestyle-redesign-health-wellbeing",
    WitnessFamily::Paper,
    "Optimizing Occupations, Habits, and Routines for Health and Well-Being With Lifestyle Redesign",
    None
);

witness_marker!(
    /// ML models predicting Windows machine vulnerability to malware infections.
    MalwarePredictionWindowsPaper,
    "domain/malware-prediction-ml-windows-machines",
    WitnessFamily::Paper,
    "Predicting Vulnerability to Malware Using Machine Learning Models: A Study on Microsoft Windows Machines",
    None
);

witness_marker!(
    /// Combinatorial graph theory applications in maturity model structure analysis.
    MaturityModelGraphTheoryPaper,
    "domain/maturity-models-combinatorial-graph-theory",
    WitnessFamily::Paper,
    "Graph Drawing Applications in Combinatorial Theory of Maturity Models",
    None
);

witness_marker!(
    /// Survey of maturity model evolution from NOLON through DevOps-era process improvement.
    MaturityModelsDevOpsSurveyPaper,
    "domain/maturity-models-survey-nolon-devops",
    WitnessFamily::Paper,
    "A Survey of Maturity Models from Nolon to DevOps and Their Applications in Process Improvement",
    None
);

witness_marker!(
    /// Occupational therapy approaches to promoting mental health in higher education settings.
    MentalHealthOtHigherEdPaper,
    "domain/mental-health-ot-higher-education",
    WitnessFamily::Paper,
    "Promoting Mental Health: Occupational Therapy in Higher Education",
    None
);

witness_marker!(
    /// Research models for metaverse UX, business readiness, and national competitiveness.
    MetaverseUserExperiencePaper,
    "domain/metaverse-ux-business-readiness-competitiveness",
    WitnessFamily::Paper,
    "Exploring the Future Metaverse: Research Models for User Experience, Business Readiness, and National Competitiveness",
    None
);

witness_marker!(
    /// Framework for pragmatic innovation balancing exploration and exploitation.
    MiddlePathInnovationPaper,
    "domain/middle-path-innovation",
    WitnessFamily::Paper,
    "Middle Path to Innovation",
    None
);

witness_marker!(
    /// MLSMM: maturity model for machine learning system security capability assessment.
    MlsmmSecurityMaturityPaper,
    "domain/mlsmm-machine-learning-security-maturity",
    WitnessFamily::Paper,
    "MLSMM: Machine Learning Security Maturity Model",
    None
);

witness_marker!(
    /// Moggi (1991): categorical semantics for monadic computation — foundation for Haskell/FP.
    MoggiMonadsComputationPaper,
    "domain/moggi-1991-notions-computation-monads",
    WitnessFamily::Paper,
    "Notions of Computation and Monads (Moggi 1991)",
    Some(1991)
);

witness_marker!(
    /// MyDigiTwin: privacy-preserving digital twin for cardiovascular risk prediction.
    MyDigiTwinPrivacyRiskPaper,
    "domain/mydigitwin-privacy-cardiovascular-risk",
    WitnessFamily::Paper,
    "MyDigiTwin: A Privacy-Preserving Framework for Personalized Cardiovascular Risk Prediction and Scenario Exploration",
    None
);

witness_marker!(
    /// NOMAD repository: FAIR principles applied to big-data-driven computational materials science.
    NomadFairMaterialsSciencePaper,
    "domain/nomad-fair-big-data-materials-science",
    WitnessFamily::Paper,
    "NOMAD: The FAIR Concept for Big-Data-Driven Materials Science",
    None
);

witness_marker!(
    /// OData v4.0 CSDL specification (errata 03): schema definition language for OData services.
    OdataV4CsdlPaper,
    "domain/odata-v4-csdl-errata03",
    WitnessFamily::Standard,
    "OData Version 4.0 Errata 03 — Common Schema Definition Language (CSDL)",
    None
);

witness_marker!(
    /// Economic model of item selling through persuasion and information disclosure.
    PersuasionSellingPaper,
    "domain/selling-item-through-persuasion",
    WitnessFamily::Paper,
    "Selling an Item through Persuasion",
    None
);

witness_marker!(
    /// Picosecond synchronization technique for mode-locked lasers in metropolitan quantum networks.
    PhotonicPulseSynchronizationPaper,
    "domain/picosecond-mode-locked-laser-quantum-networks",
    WitnessFamily::Paper,
    "Picosecond Synchronization of Mode-Locked Lasers for Metropolitan-Scale Quantum Networks",
    None
);

witness_marker!(
    /// US government PQC migration guidance and transition planning for 2024.
    PostQuantumCryptographyTransitionPaper,
    "domain/pqc-government-transition-2024",
    WitnessFamily::Paper,
    "Government Transition to Post-Quantum Cryptography (2024)",
    Some(2024)
);

witness_marker!(
    /// Digital will framework for lawful posthumous personal data management.
    PosthumousDataManagementPaper,
    "domain/posthumous-data-management-digital-will",
    WitnessFamily::Paper,
    "Beyond Life: A Digital Will Solution for Posthumous Data Management",
    None
);

witness_marker!(
    /// Hunt & Thomas: Pragmatic Programmer 20th ed — timeless software engineering practices.
    PragmaticProgrammerPaper,
    "domain/pragmatic-programmer-20th-anniversary",
    WitnessFamily::Paper,
    "The Pragmatic Programmer: Your Journey to Mastery (20th Anniversary Edition)",
    None
);

witness_marker!(
    /// Maturity model-based actuarial approach to pricing cyber insurance for systems.
    PricingCyberInsuranceMaturityPaper,
    "domain/pricing-cyber-insurance-maturity-models",
    WitnessFamily::Paper,
    "Pricing Cyber-Insurance for Systems via Maturity Models",
    None
);

witness_marker!(
    /// Industry support analysis for pricing-driven DevOps practices in SaaS companies.
    PricingDrivenDevOpsPaper,
    "domain/pricing-driven-devops-saas-industry-support",
    WitnessFamily::Paper,
    "Racing the Market: An Industry Support Analysis for Pricing-Driven DevOps in SaaS",
    None
);

witness_marker!(
    /// Empirical study of privacy norm expression and expectations through web form design.
    PrivacyNormsWebFormsPaper,
    "domain/privacy-norms-web-forms",
    WitnessFamily::Paper,
    "Understanding Privacy Norms through Web Forms",
    None
);

witness_marker!(
    /// Framework and metrics for measuring program control effectiveness in large initiatives.
    ProgramControlsEffectivenessPaper,
    "domain/program-controls-effectiveness-framework",
    WitnessFamily::Paper,
    "Program Controls Effectiveness Measurement Framework & Metrics",
    None
);

witness_marker!(
    /// Programming Phoenix LiveView: real-time server-side UIs without JavaScript frameworks.
    ProgrammingPhoenixLiveViewPaper,
    "domain/programming-phoenix-liveview",
    WitnessFamily::Paper,
    "Programming Phoenix LiveView",
    None
);

witness_marker!(
    /// Proof-bound approach to enterprise change management and verification.
    ProofBoundEnterpriseChangePaper,
    "domain/proof-bound-enterprise-change",
    WitnessFamily::Paper,
    "Proof-Bound Enterprise Change",
    None
);

witness_marker!(
    /// Study of how pseudo-automation reconfigures labor roles in frontline retail environments.
    PseudoAutomationLaborPaper,
    "domain/pseudo-automation-labor-frontline-retail",
    WitnessFamily::Paper,
    "Pseudo-Automation: How Labor-Offsetting Technologies Reconfigure Roles and Relationships in Frontline Retail Work",
    None
);

witness_marker!(
    /// Maturity model for public administration as open translation data providers.
    PublicAdminOpenTranslationMaturityPaper,
    "domain/public-admin-open-translation-maturity",
    WitnessFamily::Paper,
    "A Maturity Model for Public Administration as Open Translation Data Providers",
    None
);

witness_marker!(
    /// QuantConnect LEAN algorithmic trading engine Python API guide.
    QuantconnectLeanEnginePaper,
    "domain/quantconnect-lean-engine-python",
    WitnessFamily::Paper,
    "QuantConnect LEAN Engine Python Guide",
    None
);

witness_marker!(
    /// Real-Time Phoenix: building scalable real-time applications with Phoenix Channels.
    RealTimePhoenixPaper,
    "domain/real-time-phoenix-channels",
    WitnessFamily::Paper,
    "Real-Time Phoenix",
    None
);

witness_marker!(
    /// Maturity model for security practices in research software development.
    ResearchSoftwareSecurityMaturityPaper,
    "domain/research-software-security-maturity",
    WitnessFamily::Paper,
    "Toward a Research Software Security Maturity Model",
    None
);

witness_marker!(
    /// Global survey-based responsible AI maturity model across international contexts.
    ResponsibleAiMaturityGlobalPaper,
    "domain/responsible-ai-maturity-global-survey",
    WitnessFamily::Paper,
    "Responsible AI in the Global Context: Maturity Model and Survey",
    None
);

witness_marker!(
    /// 2024 SBOM frequently asked questions — software bill of materials guidance.
    SbomFaq2024Paper,
    "domain/sbom-faq-2024",
    WitnessFamily::Standard,
    "SBOM FAQ 2024",
    None
);

witness_marker!(
    /// Official Scrum@Scale framework guide for scaling agile across large organizations.
    ScrumAtScalePaper,
    "domain/official-scrum-at-scale-guide",
    WitnessFamily::Standard,
    "Official Scrum@Scale Guide",
    None
);

witness_marker!(
    /// Service colonies: autonomous cooperative service architecture style for distributed systems.
    ServiceColoniesArchitecturePaper,
    "domain/service-colonies-autonomous-cooperative-architecture",
    WitnessFamily::Paper,
    "Service Colonies: A Novel Architectural Style for Developing Software Systems with Autonomous and Cooperative Services",
    None
);

witness_marker!(
    /// Session type theory applied to TCP transport layer protocol verification.
    SessionTypesTcpPaper,
    "domain/session-types-transport-layer-tcp",
    WitnessFamily::Paper,
    "Session Types for the Transport Layer: Towards an Implementation of TCP*",
    None
);

witness_marker!(
    /// Signal theory framework for optimal intent encoding in communication system design.
    SignalTheoryArchitecturePaper,
    "domain/signal-theory-optimal-intent-encoding",
    WitnessFamily::Paper,
    "Signal Theory: The Architecture of Optimal Intent Encoding in Communication Systems",
    None
);

witness_marker!(
    /// Derrida's Signature Event Context: deconstruction of speech act theory and iterability.
    SignatureEventContextPaper,
    "domain/signature-event-context-derrida",
    WitnessFamily::Paper,
    "Signature Event Context (Derrida)",
    None
);

witness_marker!(
    /// Simulation as empirical software engineering research method — teaching curriculum.
    SimulationResearchMethodPaper,
    "domain/simulation-research-method-software-engineering",
    WitnessFamily::Paper,
    "Teaching Simulation as a Research Method in Empirical Software Engineering",
    None
);

witness_marker!(
    /// Comparative syntactic and semantic analysis of software testing terminology glossaries.
    SoftwareTestingGlossaryPaper,
    "domain/software-testing-glossary-syntactic-semantic",
    WitnessFamily::Paper,
    "Supplementary Results of a Comparative Syntactic and Semantic Study of Terms for Software Testing Glossaries",
    None
);

witness_marker!(
    /// Architecture process maturity model for SPL engineering organizations.
    SplArchitectureMaturityPaper,
    "domain/spl-architecture-process-maturity",
    WitnessFamily::Paper,
    "An Architecture Process Maturity Model of Software Product Line Engineering",
    None
);

witness_marker!(
    /// Business maturity model for software product line (SPL) engineering capability.
    SplMaturityModelPaper,
    "domain/spl-business-maturity-model",
    WitnessFamily::Paper,
    "A Business Maturity Model of Software Product Line Engineering",
    None
);

witness_marker!(
    /// SRE-HIMM: requirements engineering maturity model for global healthcare information systems.
    SreHimmHealthcareMaturityPaper,
    "domain/sre-himm-healthcare-requirements-maturity",
    WitnessFamily::Paper,
    "Software Requirements Engineering Healthcare Implementation Maturity Model (SRE-HIMM) for Global Health-Care Information System",
    None
);

witness_marker!(
    /// STIX v2.1 OASIS standard: Structured Threat Information eXpression for cyber threat intelligence.
    StixV21Paper,
    "domain/stix-v2-1-oasis-standard",
    WitnessFamily::Standard,
    "STIX Version 2.1 (OASIS Standard)",
    Some(2021)
);

witness_marker!(
    /// Reliability engineering challenges and innovations in Industry 4.0 environments.
    SystemReliabilityIndustry40Paper,
    "domain/system-reliability-engineering-industry-4-0",
    WitnessFamily::Paper,
    "System Reliability Engineering in the Age of Industry 4.0: Challenges and Innovations",
    None
);

witness_marker!(
    /// Systematic exploration of system resiliency properties and supporting design methods.
    SystemResiliencyDesignPaper,
    "domain/system-resiliency-design-methods",
    WitnessFamily::Paper,
    "Exploring System Resiliency and Supporting Design Methods",
    None
);

witness_marker!(
    /// TDD maturity model constructed from anti-patterns framework analysis.
    TddMaturityAntiPatternsPaper,
    "domain/tdd-maturity-model-anti-patterns",
    WitnessFamily::Paper,
    "Towards a TDD Maturity Model through an Anti-patterns Framework",
    None
);

witness_marker!(
    /// Spatial-network model of technology adoption and network externality effects in finance.
    TechnologyAdoptionNetworkExternalitiesPaper,
    "domain/technology-adoption-network-externalities-finance",
    WitnessFamily::Paper,
    "Technology Adoption and Network Externalities in Financial Systems: A Spatial-Network Approach",
    None
);

witness_marker!(
    /// Integrated IS security risk management and EA model using TOGAF/ArchiMate/IAF/DoDAF.
    TogafArchimateIafDodafPaper,
    "domain/togaf-archimate-iaf-dodaf-security-ea",
    WitnessFamily::Paper,
    "An Integrated Conceptual Model for Information System Security Risk Management and Enterprise Architecture Management based on TOGAF, ArchiMate, IAF and DoDAF",
    None
);

witness_marker!(
    /// TOGAF enterprise architecture described through SEMAT Essence Kernel practices.
    TogafSematKernelPaper,
    "domain/togaf-semat-essence-kernel",
    WitnessFamily::Paper,
    "A Brief TOGAF Description using SEMAT Essence Kernel",
    None
);

witness_marker!(
    /// Factors affecting portability of tokenized assets across distributed ledger platforms.
    TokenizedAssetPortabilityPaper,
    "domain/tokenized-assets-portability-distributed-ledgers",
    WitnessFamily::Paper,
    "Factors in the Portability of Tokenized Assets on Distributed Ledgers",
    None
);

witness_marker!(
    /// Review of total quality leadership practices in TQM for industrial management.
    TotalQualityLeadershipPaper,
    "domain/total-quality-leadership-tqm-practices",
    WitnessFamily::Paper,
    "A Review on Total Quality Leadership in TQM Practices: Industrial Management and Organizations",
    None
);

witness_marker!(
    /// TPOT2: evolutionary AutoML pipeline optimization using genetic programming.
    Tpot2AutomlPaper,
    "domain/tpot2-automl-evolutionary-optimization",
    WitnessFamily::Paper,
    "TPOT2: Next Generation Automated Machine Learning (AutoML) via Evolutionary Optimization",
    None
);

witness_marker!(
    /// Trusted CI experiences in cybersecurity services supporting open science infrastructure.
    TrustedCiCybersecurityPaper,
    "domain/trusted-ci-cybersecurity-open-science",
    WitnessFamily::Paper,
    "Trusted CI Experiences in Cybersecurity and Service to Open Science",
    None
);

witness_marker!(
    /// Technology roadmapping for green+digital twin transition in multinational corporations.
    TwinTransitionMultinationalPaper,
    "domain/twin-transition-green-digital-roadmapping",
    WitnessFamily::Paper,
    "Orchestrating the Twin Transition in Multinational Corporations: Technology Roadmapping for Green and Digital Global Business Services",
    None
);

witness_marker!(
    /// Type-safe process evidence engineering: formal Rust type law for process mining compat.
    TypeSafeProcessEvidencePaper,
    "domain/type-safe-process-evidence-engineering",
    WitnessFamily::Paper,
    "Type-Safe Process-Evidence Engineering",
    None
);

witness_marker!(
    /// Market-based replanning algorithm for safety-critical UAV swarm search and rescue.
    UavSwarmSearchRescuePaper,
    "domain/uav-swarm-market-replanning-search-rescue",
    WitnessFamily::Paper,
    "Market-Based Replanning for Safety-Critical UAV Swarms in Search and Rescue Missions",
    None
);

witness_marker!(
    /// Software architecture maturity model for ULS system interoperability improvement.
    UlsSystemsInteroperabilityMaturityPaper,
    "domain/uls-systems-architecture-maturity-interoperability",
    WitnessFamily::Paper,
    "Towards a Software Architecture Maturity Model for Improving Ultra-Large-Scale Systems Interoperability",
    None
);

witness_marker!(
    /// UPCASE: self-assessment method for usability process capability in small organizations.
    UpcaseUsabilityMaturityPaper,
    "domain/upcase-usability-process-maturity-small-org",
    WitnessFamily::Paper,
    "UPCASE: A Method for Self-Assessing the Capability of the Usability Process in Small Organizations",
    None
);

witness_marker!(
    /// HMM coupling-based USDCHF trading strategy filtered by gold market dynamics.
    UsdChfGoldHmmPaper,
    "domain/trading-usdchf-gold-hmm-coupling",
    WitnessFamily::Paper,
    "Trading USDCHF Filtered by Gold Dynamics via HMM Coupling",
    None
);

witness_marker!(
    /// Virtual organization breeding methodology for collaborative network architecture modeling.
    VirtualOrgBreedingMethodPaper,
    "domain/virtual-org-architecture-breeding-methodology",
    WitnessFamily::Paper,
    "Modeling Virtual Organization Architecture with the Virtual Organization Breeding Methodology",
    None
);

witness_marker!(
    /// W6H framework for ordering stakeholder viewpoints in holistic enterprise architecture.
    W6HEnterpriseArchitecturePaper,
    "domain/w6h-stakeholder-viewpoint-enterprise-architecture",
    WitnessFamily::Paper,
    "Ordering Stakeholder Viewpoint Concerns for Holistic and Incremental Enterprise Architecture: The W6H Framework",
    None
);

witness_marker!(
    /// Composition search tree approach for automated web service composition.
    WebServiceCompositionSearchPaper,
    "domain/web-service-composition-search-tree",
    WitnessFamily::Paper,
    "Usages of Composition Search Tree in Web Service Composition",
    None
);

witness_marker!(
    /// Empirical study: test metric combinations as predictors of software project success.
    WhichTestMetricsProjectSuccessPaper,
    "domain/test-metrics-software-project-success",
    WitnessFamily::Paper,
    "Which Combination of Test Metrics Can Predict Success of a Software Project? A Case Study in a Year-Long Project Course",
    None
);

witness_marker!(
    /// Wolpert & Macready (1997): no free lunch theorems — all optimization algorithms equivalent in aggregate.
    WolpertMaceadyNoFreeLunchPaper,
    "domain/wolpert-macready-1997-no-free-lunch",
    WitnessFamily::Paper,
    "No Free Lunch Theorems for Optimization (Wolpert & Macready 1997)",
    Some(1997)
);

witness_marker!(
    /// Autonomous identity-based threat segmentation for zero trust network architecture.
    ZeroTrustIdentitySegmentationPaper,
    "domain/zero-trust-identity-threat-segmentation",
    WitnessFamily::Paper,
    "Autonomous Identity-Based Threat Segmentation in Zero Trust Architectures",
    None
);

witness_marker!(
    /// Zuchongzhi 3.0: 105-qubit quantum computational advantage benchmark paper.
    ZuchongzhiQuantumAdvantage105QubitPaper,
    "domain/zuchongzhi-3-0-quantum-computational-advantage",
    WitnessFamily::Paper,
    "Establishing a New Benchmark in Quantum Computational Advantage with 105-qubit Zuchongzhi 3.0 Processor",
    Some(2024)
);
