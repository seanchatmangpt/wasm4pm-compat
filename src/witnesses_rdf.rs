












//! RDF/SPARQL/KG witness markers — compiled from `ontology/witnesses-rdf.ttl`.
//!
//! Every entry here is derived from a `compat:WitnessMarker` tagged
//! `compat:paperCategory "rdf"` in the ontology.
//!
//! To add a paper: declare a `compat:WitnessMarker` with
//! `compat:paperCategory "rdf"` in
//! `ontology/witnesses-rdf.ttl`, then run
//! `cargo make ggen-witnesses-rdf`.
//!
//! Structure-only authority labels. Graduate to `wasm4pm` when the
//! paper's algorithm must actually execute.


use crate::witness::WitnessFamily;


witness_marker!(
    /// AGENTICT2S: agentic multi-model Text-to-SPARQL for heterogeneous circular economy KGs.
    AgentIct2sText2SparqlPaper,
    "rdf/agentict2s-text2sparql-heterogeneous-kg",
    WitnessFamily::Paper,
    "AGENTICT2S: Robust Text-to-SPARQL via Agentic Collaborative Reasoning over Heterogeneous Knowledge Graphs for the Circular Economy",
    None
);

witness_marker!(
    /// ARUQULA: ReAct-based LLM Text2SPARQL with KG exploration utilities.
    AruqulaLlmText2SparqlPaper,
    "rdf/aruqula-llm-text2sparql-react-kg",
    WitnessFamily::Paper,
    "ARUQULA: An LLM-based Text2SPARQL Approach using ReAct and Knowledge Graph Exploration Utilities",
    None
);

witness_marker!(
    /// AutoRDF2GML: bridge between RDF knowledge graphs and graph machine learning frameworks.
    AutoRdf2GmlPaper,
    "rdf/autordf2gml-rdf-graph-machine-learning",
    WitnessFamily::Paper,
    "AutoRDF2GML: Facilitating RDF Integration in Graph Machine Learning",
    None
);

witness_marker!(
    /// Automatic mapping of AutomationML files to ontologies enabling SPARQL queries and SHACL validation.
    AutomationMlOntologyPaper,
    "rdf/automationml-ontology-graph-queries-validation",
    WitnessFamily::Paper,
    "Automatic Mapping of AutomationML Files to Ontologies for Graph Queries and Validation",
    None
);

witness_marker!(
    /// BARQ: vectorized SPARQL execution engine for high-performance RDF querying.
    BarqVectorizedSparqlPaper,
    "rdf/barq-vectorized-sparql-query-engine",
    WitnessFamily::Paper,
    "BARQ: A Vectorized SPARQL Query Execution Engine",
    None
);

witness_marker!(
    /// BDI ontology formalising mental states (beliefs, desires, intentions) for agent modelling.
    BdiOntologyAgencyPaper,
    "rdf/bdi-ontology-mental-reality-agency",
    WitnessFamily::Paper,
    "The Belief-Desire-Intention Ontology for Modelling Mental Reality and Agency",
    None
);

witness_marker!(
    /// Citation processing system for parking and administrative citations — domain application.
    CitationProcessingAdminPaper,
    "rdf/citation-processing-parking-administrative",
    WitnessFamily::Paper,
    "Citation Processing Center: Parking and Administrative Citations",
    None
);

witness_marker!(
    /// Condensed RDF representations enabling compact graph versioning and change tracking.
    CondensedRdfGraphVersioningPaper,
    "rdf/condensed-rdf-graph-versioning",
    WitnessFamily::Paper,
    "Condensed Representation of RDF and its Application on Graph Versioning",
    None
);

witness_marker!(
    /// Consistent query answering (CQA) semantics when querying RDF data under SHACL constraints.
    ConsistentQueryShaclPaper,
    "rdf/consistent-query-answering-shacl",
    WitnessFamily::Paper,
    "Consistent Query Answering over SHACL Constraints",
    None
);

witness_marker!(
    /// CypherBench: benchmark for Cypher-based retrieval over full-scale KGs with LLMs.
    CypherBenchFullScaleKgPaper,
    "rdf/cypherbench-llm-full-scale-kg",
    WitnessFamily::Paper,
    "CypherBench: Towards Precise Retrieval over Full-scale Modern Knowledge Graphs in the LLM Era",
    None
);

witness_marker!(
    /// Survey of data spaces: concepts, applications, governance, and future research directions.
    DataSpaceLandscapePaper,
    "rdf/data-space-landscape-concepts-applications",
    WitnessFamily::Paper,
    "Navigating the Data Space Landscape: Concepts, Applications, and Future Directions",
    None
);

witness_marker!(
    /// RDF-based conflict-tolerant deontic scheme for handling irresolvable semantic web conflicts.
    DeonticRdfConflictPaper,
    "rdf/deontic-rdf-conflict-tolerant-scheme",
    WitnessFamily::Paper,
    "Handling Irresolvable Conflicts in the Semantic Web: an RDF-based Conflict-Tolerant Version of the Deontic Traditional Scheme",
    None
);

witness_marker!(
    /// Multi-region harmonized KG for data-driven electricity management and analysis.
    ElectricityManagementKgPaper,
    "rdf/electricity-management-kg-harmonized-data",
    WitnessFamily::Paper,
    "Towards Data-Driven Electricity Management: Multi-Region Harmonized Data and Knowledge Graph",
    None
);

witness_marker!(
    /// Explainable NL framework for audience targeting and notification in enterprise communication.
    ExplainableNlEnterprisePaper,
    "rdf/explainable-nl-enterprise-communication",
    WitnessFamily::Paper,
    "An Explainable Natural Language Framework for Identifying and Notifying Target Audiences In Enterprise Communication",
    None
);

witness_marker!(
    /// FIRESPARQL: LLM-driven SPARQL generation for scholarly knowledge graph query interfaces.
    FireSparqlScholarlyKgPaper,
    "rdf/firesparql-llm-sparql-scholarly-kg",
    WitnessFamily::Paper,
    "FIRESPARQL: A LLM-based Framework for SPARQL Query Generation over Scholarly Knowledge Graphs",
    None
);

witness_marker!(
    /// Fuzzy embeddings for ontology exploration with visual query-building interface.
    FuzzyOntologyEmbeddingPaper,
    "rdf/fuzzy-ontology-embeddings-visual-query",
    WitnessFamily::Paper,
    "Fuzzy Ontology Embeddings and Visual Query Building for Ontology Exploration",
    None
);

witness_marker!(
    /// Workflow for crosswalking metadata in GLAM (galleries, libraries, archives, museums).
    GlamMetadataCrosswalkPaper,
    "rdf/glam-metadata-crosswalk-workflow",
    WitnessFamily::Paper,
    "A Workflow for GLAM Metadata Crosswalk",
    None
);

witness_marker!(
    /// Native GraphQL-to-RDF execution via multi-way join decomposition.
    GraphQlRdfMultiwayJoinPaper,
    "rdf/graphql-rdf-multiway-joins",
    WitnessFamily::Paper,
    "Native Execution of GraphQL Queries over RDF Graphs Using Multi-way Joins",
    None
);

witness_marker!(
    /// GRASP: framework for generic reasoning and SPARQL query generation across heterogeneous KGs.
    GraspSparqlGenerationPaper,
    "rdf/grasp-generic-reasoning-sparql-kg",
    WitnessFamily::Paper,
    "GRASP: Generic Reasoning And SPARQL Generation across Knowledge Graphs",
    None
);

witness_marker!(
    /// Agent learning in hypermedia environments using semantic web affordances.
    HypermediaGameLearningPaper,
    "rdf/hypermedia-game-learning-tic-tac-toe",
    WitnessFamily::Paper,
    "Do you want to play a game? Learning to play Tic-Tac-Toe in Hypermedia Environments",
    None
);

witness_marker!(
    /// InteracSPARQL: interactive SPARQL refinement via natural language explanations.
    InteracSparqlRefinementPaper,
    "rdf/interacsparql-query-refinement-nl",
    WitnessFamily::Paper,
    "InteracSPARQL: An Interactive System for SPARQL Query Refinement Using Natural Language Explanations",
    None
);

witness_marker!(
    /// Jelly-Patch: compact binary format for streaming RDF dataset change records.
    JellyPatchRdfChangePaper,
    "rdf/jelly-patch-rdf-dataset-changes",
    WitnessFamily::Paper,
    "Jelly-Patch: a Fast Format for Recording Changes in RDF Datasets",
    None
);

witness_marker!(
    /// Systematic procedure model for constructing industry-grade knowledge graphs.
    KgBuildingIndustryPaper,
    "rdf/kg-building-procedure-industry-applications",
    WitnessFamily::Paper,
    "Procedure Model for Building Knowledge Graphs for Industry Applications",
    None
);

witness_marker!(
    /// KG-ER: conceptual schema language bridging entity-relationship and knowledge graph modelling.
    KgErConceptualSchemaPaper,
    "rdf/kg-er-conceptual-schema-language",
    WitnessFamily::Paper,
    "The KG-ER Conceptual Schema Language",
    None
);

witness_marker!(
    /// KGpipe: automated pipeline generation and evaluation for data integration into KGs.
    KgpipeDataIntegrationPaper,
    "rdf/kgpipe-pipeline-data-integration-kg",
    WitnessFamily::Paper,
    "KGpipe: Generation and Evaluation of Pipelines for Data Integration into Knowledge Graphs",
    None
);

witness_marker!(
    /// KIF: Wikidata-grounded framework for integrating heterogeneous knowledge sources.
    KifWikidataIntegrationPaper,
    "rdf/kif-wikidata-heterogeneous-kg-integration",
    WitnessFamily::Paper,
    "KIF: A Wikidata-Based Framework for Integrating Heterogeneous Knowledge Sources",
    None
);

witness_marker!(
    /// KG-grounded self-correction mechanism for LLM hallucination via structured memory graphs.
    KnowledgeAwareSelfCorrectionPaper,
    "rdf/knowledge-aware-self-correction-llm-kg",
    WitnessFamily::Paper,
    "Knowledge-Aware Self-Correction in Language Models via Structured Memory Graphs",
    None
);

witness_marker!(
    /// Knowledge-based control plane architecture for intelligent network swarm management.
    KnowledgeControlPlaneSwarmPaper,
    "rdf/knowledge-control-plane-intelligent-swarm",
    WitnessFamily::Paper,
    "Towards an Implementation of the Knowledge-Based Control Plane for Intelligent Swarm Networks",
    None
);

witness_marker!(
    /// Ontology design patterns for reusable knowledge modelling.
    KnowledgePatternsPaper,
    "rdf/knowledge-patterns",
    WitnessFamily::Paper,
    "Knowledge Patterns",
    None
);

witness_marker!(
    /// Legislative system modelling in property graphs enabling legal pattern detection.
    LegislativePropertyGraphPaper,
    "rdf/legislative-property-graph-pattern-detection",
    WitnessFamily::Paper,
    "Modelling Legislative Systems into Property Graphs to Enable Advanced Pattern Detection",
    None
);

witness_marker!(
    /// LLM-enhanced conversational interfaces for linked data retrieval and interaction.
    LinkedDataLlmConversationalPaper,
    "rdf/linked-data-llm-conversational-ui",
    WitnessFamily::Paper,
    "Towards Enhancing Linked Data Retrieval in Conversational UIs using Large Language Models",
    None
);

witness_marker!(
    /// LLM-KG-Bench 3.0: benchmark evaluating LLM semantic technology capabilities over KGs.
    LlmKgBench30Paper,
    "rdf/llm-kg-bench-3-0-semantic-technology",
    WitnessFamily::Paper,
    "LLM-KG-Bench 3.0: A Compass for Semantic Technology Capabilities in the Ocean of LLMs",
    None
);

witness_marker!(
    /// LLM-generated natural language explanations for component-based KGQA system answers.
    LlmKgExplanationsPaper,
    "rdf/llm-kg-explanations-component-based-qa",
    WitnessFamily::Paper,
    "Towards LLM-generated Explanations for Component-based Knowledge Graph Question Answering Systems",
    None
);

witness_marker!(
    /// KG-comparison method for detecting LLM plagiarism of training data sources.
    LlmsPlagiarizeKgSourcePaper,
    "rdf/llms-plagiarize-kg-training-data",
    WitnessFamily::Paper,
    "LLMs Plagiarize: Ensuring Responsible Sourcing of Large Language Model Training Data Through Knowledge Graph Comparison",
    None
);

witness_marker!(
    /// MCP specification (mcp.pdf): protocol for connecting AI assistants to data sources and tools.
    McpSpecificationPaper,
    "rdf/mcp-model-context-protocol-spec",
    WitnessFamily::Standard,
    "Model Context Protocol (MCP) Specification",
    None
);

witness_marker!(
    /// Metadata profiling model for multidimensional data ecosystem sources.
    MetadataModelProfilingPaper,
    "rdf/metadata-model-profiling-multidimensional",
    WitnessFamily::Paper,
    "A Metadata Model for Profiling Multidimensional Sources in Data Ecosystems",
    None
);

witness_marker!(
    /// Comparative study of metadata representation models for KG embedding quality.
    MetadataRepresentationKgePaper,
    "rdf/metadata-representation-kg-embeddings",
    WitnessFamily::Paper,
    "Comparison of Metadata Representation Models for Knowledge Graph Embeddings",
    None
);

witness_marker!(
    /// Multi-agent GraphRAG translating NL questions to Cypher for property graph querying.
    MultiAgentGraphRagCypherPaper,
    "rdf/multi-agent-graphrag-text-cypher",
    WitnessFamily::Paper,
    "Multi-Agent GraphRAG: A Text-to-Cypher Framework for Labeled Property Graphs",
    None
);

witness_marker!(
    /// Overview of NoSQL graph database systems: models, architectures, and query languages.
    NoSqlGraphDatabasesPaper,
    "rdf/nosql-graph-databases-overview",
    WitnessFamily::Paper,
    "NoSQL Graph Databases: an Overview",
    None
);

witness_marker!(
    /// OCEP: ontology-driven complex event processing for big data healthcare decision support.
    OcepOntologyHealthcareDecisionPaper,
    "rdf/ocep-ontology-complex-event-healthcare",
    WitnessFamily::Paper,
    "OCEP: An Ontology-Based Complex Event Processing Framework for Healthcare Decision Support in Big Data Analytics",
    None
);

witness_marker!(
    /// OntoAligner extension integrating KG embedding methods for ontology alignment.
    OntoAlignerKgEmbeddingPaper,
    "rdf/ontoaligner-kg-embedding-aligners",
    WitnessFamily::Paper,
    "OntoAligner Meets Knowledge Graph Embedding Aligners",
    None
);

witness_marker!(
    /// Ontology-based runtime feedback for adaptive multi-agent manufacturing control.
    OntologyManufacturingMultiAgentPaper,
    "rdf/ontology-manufacturing-multi-agent-feedback",
    WitnessFamily::Paper,
    "Ontology-Based Feedback to Improve Runtime Control for Multi-Agent Manufacturing Systems",
    None
);

witness_marker!(
    /// Ontology-based structuring and analysis of public procurement contracts for transparency.
    OntologyProcurementContractsPaper,
    "rdf/ontology-procurement-contracts-north-macedonian",
    WitnessFamily::Paper,
    "Ontology-Based Structuring and Analysis of North Macedonian Public Procurement Contracts",
    None
);

witness_marker!(
    /// Ontology-driven M2M transformation enabling interoperable workflow specification conversion.
    OntologyWorkflowTransformationPaper,
    "rdf/ontology-workflow-model-transformation",
    WitnessFamily::Paper,
    "Ontology-Driven Model-to-Model Transformation of Workflow Specifications",
    None
);

witness_marker!(
    /// Task planning using OWL-DL ontologies for expressive background knowledge integration.
    OwlDlPlanningPaper,
    "rdf/owl-dl-ontology-planning-extended",
    WitnessFamily::Paper,
    "Planning with OWL-DL Ontologies (Extended Version)",
    None
);

witness_marker!(
    /// Semantic unit approach to OWL expressivity for FAIR and cognitively accessible KGs.
    OwlExpressivityFairPaper,
    "rdf/owl-expressivity-fair-cognitive-interoperability",
    WitnessFamily::Paper,
    "Rethinking OWL Expressivity: Semantic Units for FAIR and Cognitively Interoperable Knowledge Graphs",
    None
);

witness_marker!(
    /// Persistent semantic data terms-of-use framework for decentralized web environments.
    PerennialSemanticTermsUsePaper,
    "rdf/perennial-semantic-data-terms-decentralized-web",
    WitnessFamily::Paper,
    "Perennial Semantic Data Terms of Use for Decentralized Web",
    None
);

witness_marker!(
    /// PKG API: tool for creating, querying, and managing personal knowledge graphs.
    PkgApiPersonalKgPaper,
    "rdf/pkg-api-personal-knowledge-graph",
    WitnessFamily::Paper,
    "PKG API: A Tool for Personal Knowledge Graph Management",
    None
);

witness_marker!(
    /// Poseidon: unified OneGraph engine supporting RDF, property graphs, and JSON natively.
    PoseidonOneGraphEnginePaper,
    "rdf/poseidon-onegraph-engine",
    WitnessFamily::Paper,
    "Poseidon: A OneGraph Engine",
    None
);

witness_marker!(
    /// Process trace querying via knowledge graphs using Notation3 reasoning rules.
    ProcessTraceKgN3Paper,
    "rdf/process-trace-querying-kg-notation3",
    WitnessFamily::Paper,
    "Process Trace Querying using Knowledge Graphs and Notation3",
    None
);

witness_marker!(
    /// Provenance tracking for data integrity in semantic web / linked data frameworks.
    ProvenanceTrackingSemanticWebPaper,
    "rdf/provenance-tracking-semantic-web",
    WitnessFamily::Paper,
    "Enhancing Data Integrity through Provenance Tracking in Semantic Web Frameworks",
    None
);

witness_marker!(
    /// PyRML: Python library for declarative RDF KG creation via RML mapping rules.
    PyRmlKgCreationPaper,
    "rdf/pyrml-kg-creation-streamlining",
    WitnessFamily::Paper,
    "Streamlining Knowledge Graph Creation with PyRML",
    None
);

witness_marker!(
    /// QLever: high-performance SPARQL engine with efficient index-based query evaluation.
    QleverSparqlEnginePaper,
    "rdf/qlever-sparql-engine-cikm-2017",
    WitnessFamily::Paper,
    "QLever: A High-Performance SPARQL Engine (CIKM 2017)",
    Some(2017)
);

witness_marker!(
    /// RAG-enhanced event knowledge base construction with proof-assistant-based reasoning.
    RagEventKbProofAssistantPaper,
    "rdf/rag-event-kb-proof-assistant-reasoning",
    WitnessFamily::Paper,
    "Reasoning with RAGged events: RAG-Enhanced Event Knowledge Base Construction and Reasoning with Proof-Assistants",
    None
);

witness_marker!(
    /// GNN-based recommender systems bridging RDF KG semantics with graph neural learning.
    RdfGnnRecommenderPaper,
    "rdf/rdf-gnn-semantic-rich-recommender",
    WitnessFamily::Paper,
    "Bridging RDF Knowledge Graphs with Graph Neural Networks for Semantically-Rich Recommender Systems",
    None
);

witness_marker!(
    /// RDFGraphGen: synthetic RDF graph generation driven by SHACL shape constraints.
    RdfGraphGenShaclPaper,
    "rdf/rdfgraphgen-shacl-shapes-generator",
    WitnessFamily::Paper,
    "RDFGraphGen: An RDF Graph Generator based on SHACL Shapes",
    None
);

witness_marker!(
    /// RDF-star2Vec: graph embedding method exploiting RDF-star reification for data mining.
    RdfStar2VecPaper,
    "rdf/rdfstar2vec-graph-embeddings-mining",
    WitnessFamily::Paper,
    "RDF-star2Vec: RDF-star Graph Embeddings for Data Mining",
    None
);

witness_marker!(
    /// RDF representation of multi-dimensional data tensors with SPARQL querying support.
    RdfTensorsSparqlPaper,
    "rdf/rdf-tensors-sparql-querying",
    WitnessFamily::Paper,
    "Representing and Querying Data Tensors in RDF and SPARQL",
    None
);

witness_marker!(
    /// Ontology-CEP-LLM integration for real-time TB health analytics.
    RealTimeHealthOntologyPaper,
    "rdf/real-time-health-ontology-cep-llm-tb",
    WitnessFamily::Paper,
    "Real-Time Health Analytics Using Ontology-Driven Complex Event Processing and LLM Reasoning: A Tuberculosis Case Study",
    None
);

witness_marker!(
    /// Structural extension to ISO:TC 37 via recursive semantic anchoring in ISO 639-2023.
    RecursiveSemanticIso639Paper,
    "rdf/recursive-semantic-anchoring-iso-639-2023",
    WitnessFamily::Paper,
    "Recursive Semantic Anchoring in ISO 639-2023: A Structural Extension to ISO:TC 37 Frameworks",
    None
);

witness_marker!(
    /// Relational-to-RDF migration via co-evaluation of SQL and SPARQL queries.
    RelationalToRdfMigrationPaper,
    "rdf/relational-to-rdf-migration-query-evaluation",
    WitnessFamily::Paper,
    "Relational to RDF Data Migration by Query Co-Evaluation",
    None
);

witness_marker!(
    /// Rule-based CEP for air quality monitoring in smart city IoT environments.
    RuleBasedCepAirQualityPaper,
    "rdf/rule-based-cep-air-quality-smart-city",
    WitnessFamily::Paper,
    "Rule based Complex Event Processing for an Air Quality Monitoring System in Smart City",
    None
);

witness_marker!(
    /// Semantic audit model for cloud engines grounded in ISO/IEC TR 3445-2022.
    SemanticAuditCloudPaper,
    "rdf/semantic-audit-cloud-iso-iec-tr-3445-2022",
    WitnessFamily::Paper,
    "A Semantic Model for Audit of Cloud Engines based on ISO:IEC TR 3445-2022",
    None
);

witness_marker!(
    /// KR-based self-indexing for semantic web endpoints using the endpoints' own knowledge.
    SemanticWebIndexingPaper,
    "rdf/eat-own-kr-semantic-web-indexing",
    WitnessFamily::Paper,
    "Eat your own KR: a KR-based approach to index Semantic Web Endpoints and Knowledge Graphs",
    None
);

witness_marker!(
    /// SHACL2FOL: first-order logic translation toolkit for decidable SHACL reasoning.
    Shacl2FolDecisionPaperRdf,
    "rdf/shacl2fol-toolkit-decision-problems",
    WitnessFamily::Paper,
    "SHACL2FOL: An FOL Toolkit for SHACL Decision Problems",
    None
);

witness_marker!(
    /// Shacl4Bib: SHACL-based custom validation profiles for bibliographic library data.
    Shacl4BibLibraryPaper,
    "rdf/shacl4bib-library-data-validation",
    WitnessFamily::Paper,
    "Shacl4Bib: Custom Validation of Library Data",
    None
);

witness_marker!(
    /// SHACL-DS: dataset-level SHACL extension for named graph and dataset validation.
    ShaclDsDatasetValidationPaper,
    "rdf/shacl-ds-extension-dataset-validation",
    WitnessFamily::Paper,
    "SHACL-DS: A SHACL Extension to Validate RDF Datasets",
    None
);

witness_marker!(
    /// SHACL validation semantics with ontology reasoning: OWL-aware shape validation and rewriting.
    ShaclOntologyRewritingPaper,
    "rdf/shacl-ontology-semantics-rewriting",
    WitnessFamily::Paper,
    "SHACL Validation in the Presence of Ontologies: Semantics and Rewriting Techniques",
    None
);

witness_marker!(
    /// Unified formal foundations for SHACL, ShEx, and PG-Schema shape languages.
    ShaclShexPgSchemaPaper,
    "rdf/common-foundations-shacl-shex-pg-schema",
    WitnessFamily::Paper,
    "Common Foundations for SHACL, ShEx, and PG-Schema",
    None
);

witness_marker!(
    /// SHACL/ShEx shape-based optimization for link traversal SPARQL queries.
    ShapeLinkTraversalQueryPaper,
    "rdf/shape-link-traversal-query-optimization",
    WitnessFamily::Paper,
    "Opportunities for Shape-based Optimization of Link Traversal Queries",
    None
);

witness_marker!(
    /// SigSPARQL: treating temporal signals as first-class SPARQL query objects over KGs.
    SigSparqlSignalKgPaper,
    "rdf/sigsparql-signals-first-class-kg",
    WitnessFamily::Paper,
    "SigSPARQL: Signals as a First-Class Citizen When Querying Knowledge Graphs",
    None
);

witness_marker!(
    /// SLM fine-tuning generalization limits for shape-based property extraction from KGs.
    SlmFinetuningShapeExtractionPaper,
    "rdf/slm-finetuning-shape-property-extraction",
    WitnessFamily::Paper,
    "Overcoming the Generalization Limits of SLM Finetuning for Shape-Based Extraction of Datatype and Object Properties",
    None
);

witness_marker!(
    /// SmartWS vision for intelligent, context-aware web services using semantic technologies.
    SmartWebServicesFuturePaper,
    "rdf/smart-web-services-future",
    WitnessFamily::Paper,
    "Smart Web Services (SmartWS): The Future of Services on the Web",
    None
);

witness_marker!(
    /// SPARQL Anything: façade-based KG construction from heterogeneous data sources via SPARQL.
    SparqlAnythingFacadePaper,
    "rdf/sparql-anything-facade-kg-construction",
    WitnessFamily::Paper,
    "Streamlining Knowledge Graph Construction with a Façade: The SPARQL Anything Project",
    None
);

witness_marker!(
    /// SPARQL CONSTRUCT in Notation3 as a semantic web rule language.
    SparqlN3ConstructRulePaper,
    "rdf/sparql-n3-construct-rule-language",
    WitnessFamily::Paper,
    "SPARQL in N3: SPARQL CONSTRUCT as a Rule Language for the Semantic Web (Extended Version)",
    None
);

witness_marker!(
    /// Spider4SSC dataset with S2CLite SPARQL-to-Cypher translation for multi-query-language benchmarking.
    Spider4SscMultiQueryPaper,
    "rdf/spider4ssc-text-multi-query-sparql-cypher",
    WitnessFamily::Paper,
    "Spider4SSC & S2CLite: A Text-to-Multi-Query-Language Dataset using Lightweight Ontology-Agnostic SPARQL to Cypher Parser",
    None
);

witness_marker!(
    /// Ta-G-T: RDF-graph-based subjectivity capture for structured table-to-text generation.
    TaGtTableTextRdfPaper,
    "rdf/ta-g-t-subjectivity-table-text-rdf",
    WitnessFamily::Paper,
    "Ta-G-T: Subjectivity Capture in Table to Text Generation via RDF Graphs",
    None
);

witness_marker!(
    /// Task-oriented GNN training strategy for accurate and efficient large-scale KG modeling.
    TaskOrientedGnnKgPaper,
    "rdf/task-oriented-gnn-large-kg",
    WitnessFamily::Paper,
    "Task-Oriented GNNs Training on Large Knowledge Graphs for Accurate and Efficient Modeling",
    None
);

witness_marker!(
    /// LLM-based taxonomy inference from tabular data for KG schema enrichment.
    TaxonomyInferenceLlmPaper,
    "rdf/taxonomy-inference-tabular-llm",
    WitnessFamily::Paper,
    "Taxonomy Inference for Tabular Data Using Large Language Models",
    None
);

witness_marker!(
    /// GUCON temporal obligation modeling using SPARQL-star / RDF-star reification.
    TemporalObligationsGuconPaper,
    "rdf/temporal-obligations-gucon-sparql-star",
    WitnessFamily::Paper,
    "Modeling and Managing Temporal Obligations in GUCON Using SPARQL-star and RDF-star",
    None
);

witness_marker!(
    /// Probabilistic temporal knowledge extraction from IoT sensor streams in smart buildings.
    TimeProbabilityIotKgPaper,
    "rdf/time-probability-iot-smart-building",
    WitnessFamily::Paper,
    "Time-Probability Dependent Knowledge Extraction in IoT-enabled Smart Building",
    None
);

witness_marker!(
    /// LLM-constructed scientific ontology for urban intermodal freight transportation decision support.
    UrbanIntermodalFreightOntologyPaper,
    "rdf/urban-intermodal-freight-ontology-llm",
    WitnessFamily::Paper,
    "Towards Next-Generation Urban Decision Support Systems through AI-Powered Construction of Scientific Ontology using Large Language Models: A Case in Optimizing Intermodal Freight Transportation",
    None
);

witness_marker!(
    /// Knowledge graph representation of web applications for semantic analysis and querying.
    WebApplicationsKgPaper,
    "rdf/web-applications-knowledge-graphs",
    WitnessFamily::Paper,
    "Representing Web Applications As Knowledge Graphs",
    None
);

witness_marker!(
    /// WEBDial: RDF-backed multi-domain statistical dialogue system.
    WebDialMultiDomainRdfPaper,
    "rdf/webdial-multi-domain-rdf-dialogue",
    WitnessFamily::Paper,
    "WEBDial: a Multi-domain, Multitask Statistical Dialogue Framework with RDF",
    None
);

witness_marker!(
    /// xpSHACL: RAG-LLM pipeline generating natural language explanations for SHACL violations.
    XpShaclExplainableRagPaper,
    "rdf/xpshacl-explainable-validation-rag",
    WitnessFamily::Paper,
    "xpSHACL: Explainable SHACL Validation using Retrieval-Augmented Generation and Large Language Models",
    None
);
