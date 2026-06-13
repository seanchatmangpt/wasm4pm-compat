












//! Cognition Breed witness markers — compiled from `ontology/witnesses-cognition.ttl`.
//!
//! Every entry here is derived from a `compat:WitnessMarker` tagged
//! `compat:paperCategory "cognition"` in the ontology.
//!
//! To add a paper: declare a `compat:WitnessMarker` with
//! `compat:paperCategory "cognition"` in
//! `ontology/witnesses-cognition.ttl`, then run
//! `cargo make ggen-witnesses-cognition`.
//!
//! Structure-only authority labels. Graduate to `wasm4pm` when the
//! paper's algorithm must actually execute.


use crate::witness::WitnessFamily;


witness_marker!(
    /// Abductive IBE — Harman (1965): inference to the best explanation.
    AbductiveIbePaper,
    "cognition/abductive-ibe-1965",
    WitnessFamily::Paper,
    "The Inference to the Best Explanation",
    Some(1965)
);

witness_marker!(
    /// Abductive LP — Kakas, Kowalski & Toni (1992): abduction in logic programs.
    AbductiveLpPaper,
    "cognition/abductive-lp-1992",
    WitnessFamily::Paper,
    "Abductive Logic Programming",
    Some(1992)
);

witness_marker!(
    /// AC-3 — Mackworth (1977): arc-consistency for constraint satisfaction.
    Ac3Paper,
    "cognition/ac3-csp-1977",
    WitnessFamily::Paper,
    "Consistency in Networks of Relations",
    Some(1977)
);

witness_marker!(
    /// ACT-R — Anderson & Lebiere (1998): cognitive architecture for human cognition.
    ActRPaper,
    "cognition/act-r-1998",
    WitnessFamily::Paper,
    "The Atomic Components of Thought",
    Some(1998)
);

witness_marker!(
    /// Allen interval algebra — Allen (1983): 13 interval relations for temporal reasoning.
    AllenTemporalIntervalsPaper,
    "cognition/allen-temporal-1983",
    WitnessFamily::Paper,
    "Maintaining Knowledge about Temporal Intervals",
    Some(1983)
);

witness_marker!(
    /// Bayesian Networks — Pearl (1988): probabilistic graphical models.
    BayesianNetworkPaper,
    "cognition/bayesian-network-1988",
    WitnessFamily::Paper,
    "Probabilistic Reasoning in Intelligent Systems",
    Some(1988)
);

witness_marker!(
    /// Belief Merging — Konieczny & Pino Pérez (2002): logical belief base merging.
    BeliefMergingPaper,
    "cognition/belief-merging-2002",
    WitnessFamily::Paper,
    "Merging Information Under Constraints: A Logical Framework",
    Some(2002)
);

witness_marker!(
    /// MDP — Bellman (1957): Markovian decision processes and dynamic programming.
    BellmanMdpPaper,
    "cognition/bellman-mdp-1957",
    WitnessFamily::Paper,
    "A Markovian Decision Process",
    Some(1957)
);

witness_marker!(
    /// Boden (1977): AI creativity and neurosis models in natural intelligence.
    BodenCreativityPaper,
    "cognition/boden-creativity-1977",
    WitnessFamily::Paper,
    "Artificial Intelligence and Natural Man",
    Some(1977)
);

witness_marker!(
    /// CBR — Aamodt & Plaza (1994): case-based reasoning foundational framework.
    CbrFoundationsPaper,
    "cognition/cbr-foundations-1994",
    WitnessFamily::Paper,
    "Case-Based Reasoning: Foundational Issues, Methodological Variations, and System Approaches",
    Some(1994)
);

witness_marker!(
    /// CDCL SAT — Marques-Silva & Sakallah (1999): conflict-driven clause learning.
    CdclSatPaper,
    "cognition/cdcl-sat-1999",
    WitnessFamily::Paper,
    "GRASP: A Search Algorithm for Propositional Satisfiability",
    Some(1999)
);

witness_marker!(
    /// Circumscription — McCarthy (1980): minimizing predicate extensions.
    CircumscriptionPaper,
    "cognition/circumscription-1980",
    WitnessFamily::Paper,
    "Circumscription — A Form of Non-Monotonic Reasoning",
    Some(1980)
);

witness_marker!(
    /// Classical AI foundations reference document covering symbolic AI history.
    ClassicalAiFoundationsMd,
    "cognition/classical-ai-foundations-overview",
    WitnessFamily::Paper,
    "Classical AI Foundations Overview",
    None
);

witness_marker!(
    /// Conceptual Dependency — Schank (1972): semantic representation of NL meaning.
    ConceptualDependencyPaper,
    "cognition/conceptual-dependency-1972",
    WitnessFamily::Paper,
    "Conceptual Dependency: A Theory of Natural Language Understanding",
    Some(1972)
);

witness_marker!(
    /// CLP — Jaffar & Lassez (1987): constraint satisfaction over logic programming.
    ConstraintLogicProgrammingPaper,
    "cognition/clp-1987",
    WitnessFamily::Paper,
    "Constraint Logic Programming",
    Some(1987)
);

witness_marker!(
    /// Construction Grammar — Goldberg (1995): form-meaning constructions.
    ConstructionGrammarPaper,
    "cognition/construction-grammar-1995",
    WitnessFamily::Paper,
    "Constructions: A Construction Grammar Approach to Argument Structure",
    Some(1995)
);

witness_marker!(
    /// Contingent planning — Russell & Norvig (2010): planning under uncertainty.
    ContingentPlanPaper,
    "cognition/contingent-plan-2010",
    WitnessFamily::Paper,
    "Artificial Intelligence: A Modern Approach (3rd ed.)",
    Some(2010)
);

witness_marker!(
    /// CTL model checking — Clarke, Emerson & Sistla (1986): temporal logic verification.
    CtlModelCheckingPaper,
    "cognition/ctl-model-checking-1986",
    WitnessFamily::Paper,
    "Automatic Verification of Finite State Concurrent Systems",
    Some(1986)
);

witness_marker!(
    /// Default Logic — Reiter (1980): non-monotonic reasoning with default rules.
    DefaultLogicPaper,
    "cognition/default-logic-1980",
    WitnessFamily::Paper,
    "A Logic for Default Reasoning",
    Some(1980)
);

witness_marker!(
    /// Dempster-Shafer — Shafer (1976): belief functions and plausibility.
    DempsterShaferPaper,
    "cognition/dempster-shafer-1976",
    WitnessFamily::Paper,
    "A Mathematical Theory of Evidence",
    Some(1976)
);

witness_marker!(
    /// DENDRAL — Feigenbaum, Buchanan & Lederberg (1971): chemical structure expert system.
    DendralPaper,
    "cognition/dendral-1971",
    WitnessFamily::Paper,
    "Applications of Artificial Intelligence for Chemical Inference",
    Some(1971)
);

witness_marker!(
    /// EL Description Logic — Baader, Brandt & Lutz (2005): tractable DL reasoning.
    ElEnvelopePaper,
    "cognition/el-description-logic-2005",
    WitnessFamily::Paper,
    "Pushing the EL Envelope",
    Some(2005)
);

witness_marker!(
    /// ELIZA — Weizenbaum (1966): pattern-matching natural language processing.
    ElizaPaper,
    "cognition/eliza-1966",
    WitnessFamily::Paper,
    "ELIZA: A Computer Program for the Study of Natural Language Communication",
    Some(1966)
);

witness_marker!(
    /// Episodic Memory — Tulving (1983): episodic vs semantic memory distinction.
    EpisodicMemoryPaper,
    "cognition/episodic-memory-1983",
    WitnessFamily::Paper,
    "Elements of Episodic Memory",
    Some(1983)
);

witness_marker!(
    /// Event Calculus — Kowalski & Sergot (1986): temporal reasoning over events.
    EventCalculusPaper,
    "cognition/event-calculus-1986",
    WitnessFamily::Paper,
    "A Logic-Based Calculus of Events",
    Some(1986)
);

witness_marker!(
    /// EBG — Mitchell, Keller & Kedar-Cabelli (1986): explanation-based learning.
    ExplanationBasedGeneralizationPaper,
    "cognition/ebl-1986",
    WitnessFamily::Paper,
    "Explanation-Based Generalization: A Unifying View",
    Some(1986)
);

witness_marker!(
    /// Frames — Minsky (1974): knowledge representation via frames and inheritance.
    FramesInheritancePaper,
    "cognition/frames-1974",
    WitnessFamily::Paper,
    "A Framework for Representing Knowledge",
    Some(1974)
);

witness_marker!(
    /// General Problem Solver — Newell & Simon (1961): means-ends analysis.
    GeneralProblemSolverPaper,
    "cognition/gps-1961",
    WitnessFamily::Paper,
    "GPS: A Program That Simulates Human Thought",
    Some(1961)
);

witness_marker!(
    /// HACKER — Sussman (1973): procedural learning and skill acquisition.
    HackerLearningPaper,
    "cognition/hacker-learning-1973",
    WitnessFamily::Paper,
    "A Computational Model of Skill Acquisition",
    Some(1973)
);

witness_marker!(
    /// Hearsay-II — Erman et al. (1980): blackboard architecture for speech understanding.
    HearsayIiPaper,
    "cognition/hearsay-ii-1980",
    WitnessFamily::Paper,
    "The Hearsay-II Speech-Understanding System",
    Some(1980)
);

witness_marker!(
    /// ILP — Quinlan (1990): inductive logic programming from relational data.
    InductiveLogicProgrammingPaper,
    "cognition/ilp-1990",
    WitnessFamily::Paper,
    "Learning Logical Definitions from Relations",
    Some(1990)
);

witness_marker!(
    /// LTL runtime monitoring — Havelund & Rosu (2001): rewriting-based runtime verification.
    LtlRuntimeMonitoringPaper,
    "cognition/ltl-runtime-monitoring-2001",
    WitnessFamily::Paper,
    "Monitoring Programs Using Rewriting",
    Some(2001)
);

witness_marker!(
    /// Fuzzy Logic — Mamdani & Assilian (1975): linguistic fuzzy control.
    MamdaniFuzzyPaper,
    "cognition/mamdani-fuzzy-1975",
    WitnessFamily::Paper,
    "An Experiment in Linguistic Synthesis with a Fuzzy Logic Controller",
    Some(1975)
);

witness_marker!(
    /// Markov Logic — Richardson & Domingos (2006): soft logic with Markov networks.
    MarkovLogicNetworksPaper,
    "cognition/markov-logic-2006",
    WitnessFamily::Paper,
    "Markov Logic Networks",
    Some(2006)
);

witness_marker!(
    /// Marr & Poggio (1976): cooperative stereo disparity computation model.
    MarrPoggioPaper,
    "cognition/marr-poggio-vision-1976",
    WitnessFamily::Paper,
    "Cooperative Computation of Stereo Disparity",
    Some(1976)
);

witness_marker!(
    /// Meta-reasoning — Cox & Raja (2011): reasoning about one's own reasoning.
    MetaReasoningPaper,
    "cognition/meta-reasoning-2011",
    WitnessFamily::Paper,
    "Metareasoning: Thinking about Thinking",
    Some(2011)
);

witness_marker!(
    /// Morphological Analysis — Zwicky (1969): systematic combinatorial problem exploration.
    MorphologicalAnalysisPaper,
    "cognition/morphological-1969",
    WitnessFamily::Paper,
    "Discovery, Invention, Research Through the Morphological Approach",
    Some(1969)
);

witness_marker!(
    /// MYCIN — Shortliffe & Buchanan (1975): certainty-factor rule-based expert system.
    MycinPaper,
    "cognition/mycin-1975",
    WitnessFamily::Paper,
    "A Model of Inexact Reasoning in Medicine",
    Some(1975)
);

witness_marker!(
    /// Naive Physics — Hayes (1979): commonsense physical reasoning.
    NaivePhysicsPaper,
    "cognition/naive-physics-1979",
    WitnessFamily::Paper,
    "The Naive Physics Manifesto",
    Some(1979)
);

witness_marker!(
    /// OC-PM route discovery — van der Aalst (2019): object-centric process mining framework.
    OcpmRouteDiscovererPaper,
    "cognition/ocpm-route-discoverer-2019",
    WitnessFamily::Paper,
    "Object-Centric Process Mining",
    Some(2019)
);

witness_marker!(
    /// Partial-order planning — McAllester & Rosenblitt (1991).
    PartialOrderPlanPaper,
    "cognition/partial-order-plan-1991",
    WitnessFamily::Paper,
    "Systematic Nonlinear Planning",
    Some(1991)
);

witness_marker!(
    /// LLM-guided PDDL planning: using language models for classical planning domains.
    PddlLlmPlanningPaper,
    "cognition/pddl-llm-planning",
    WitnessFamily::Paper,
    "LLM Planning with PDDL",
    None
);

witness_marker!(
    /// POMDP — Kaelbling, Littman & Cassandra (1998): planning under partial observability.
    PomdpPaper,
    "cognition/pomdp-1998",
    WitnessFamily::Paper,
    "Planning and Acting in Partially Observable Stochastic Domains",
    Some(1998)
);

witness_marker!(
    /// ProbLog — De Raedt, Kimmig & Toivonen (2007): probabilistic logic programs.
    ProblogPaper,
    "cognition/problog-2007",
    WitnessFamily::Paper,
    "ProbLog: A Probabilistic Prolog",
    Some(2007)
);

witness_marker!(
    /// Prolog — Kowalski (1974): logic programming via predicate calculus.
    PrologPaper,
    "cognition/prolog-1974",
    WitnessFamily::Paper,
    "Predicate Logic as Programming Language",
    Some(1974)
);

witness_marker!(
    /// Q-Learning — Watkins & Dayan (1992): model-free reinforcement learning.
    QLearnPaper,
    "cognition/q-learning-1992",
    WitnessFamily::Paper,
    "Q-Learning",
    Some(1992)
);

witness_marker!(
    /// Qualitative Physics — de Kleer & Brown (1984): device-centred qualitative simulation.
    QualitativePhysicsPaper,
    "cognition/qualitative-physics-1984",
    WitnessFamily::Paper,
    "A Qualitative Physics Based on Confluences",
    Some(1984)
);

witness_marker!(
    /// Resolution — Robinson (1965): unification-based automated theorem proving.
    RobinsonResolutionPaper,
    "cognition/robinson-resolution-1965",
    WitnessFamily::Paper,
    "A Machine-Oriented Logic Based on the Resolution Principle",
    Some(1965)
);

witness_marker!(
    /// Scripts/SAM — Schank & Abelson (1977): story understanding via event scripts.
    ScriptTheoryPaper,
    "cognition/script-sam-1977",
    WitnessFamily::Paper,
    "Scripts, Plans, Goals and Understanding",
    Some(1977)
);

witness_marker!(
    /// SHOP2 HTN planning — Nau et al. (2003): hierarchical task network planning.
    Shop2Paper,
    "cognition/shop2-2003",
    WitnessFamily::Paper,
    "SHOP2: An HTN Planning System",
    Some(2003)
);

witness_marker!(
    /// Situation Calculus — Reiter (1991): actions and change in first-order logic.
    SituationCalculusPaper,
    "cognition/situation-calculus-1991",
    WitnessFamily::Paper,
    "A Logic for Default Reasoning (Situation Calculus)",
    Some(1991)
);

witness_marker!(
    /// SOAR — Laird, Newell & Rosenbloom (1987): unified cognitive architecture.
    SoarPaper,
    "cognition/soar-1987",
    WitnessFamily::Paper,
    "Soar: An Architecture for General Intelligence",
    Some(1987)
);

witness_marker!(
    /// Answer Set Programming — Gelfond & Lifschitz (1988): stable model semantics.
    StableModelSemanticsPaper,
    "cognition/asp-1988",
    WitnessFamily::Paper,
    "The Stable Model Semantics for Logic Programming",
    Some(1988)
);

witness_marker!(
    /// STRIPS — Fikes & Nilsson (1971): precondition/add/delete planning operators.
    StripsPaper,
    "cognition/strips-1971",
    WitnessFamily::Paper,
    "STRIPS: A New Approach to the Application of Theorem Proving",
    Some(1971)
);

witness_marker!(
    /// SME — Falkenhainer, Forbus & Gentner (1989): structural analogy mapping.
    StructureMappingEnginePaper,
    "cognition/sme-analogy-1989",
    WitnessFamily::Paper,
    "The Structure-Mapping Engine: Algorithm and Examples",
    Some(1989)
);

witness_marker!(
    /// Tableaux — Smullyan (1968): analytic tableaux for first-order logic.
    TableauxPaper,
    "cognition/tableaux-1968",
    WitnessFamily::Paper,
    "First-Order Logic",
    Some(1968)
);

witness_marker!(
    /// TRIZ — Altshuller (1984): theory of inventive problem solving.
    TrizPaper,
    "cognition/triz-1984",
    WitnessFamily::Paper,
    "Creativity as an Exact Science",
    Some(1984)
);

witness_marker!(
    /// Version Space — Mitchell (1982): hypothesis generalization via boundary sets.
    VersionSpacePaper,
    "cognition/version-space-1982",
    WitnessFamily::Paper,
    "Generalization as Search",
    Some(1982)
);
