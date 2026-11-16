
//! Cognition breed witness markers — compiled from `breed-vocabulary.ttl`.
//!
//! Every entry here is derived from a `compat:CognitionBreed` instance in the ontology.
//! To add a breed witness: declare the breed in `breed-vocabulary.ttl`, run
//! `ggen sync --rule cognition-witnesses`, this file updates.
//!
//! These are structure-only authority labels. A witness names the founding paper
//! of a cognition breed algorithm. It does not implement the algorithm. Graduate
//! to `wasm4pm` when the breed must actually manufacture an artifact.

use crate::witness::WitnessFamily;


witness_marker!(
    /// Harman, G. H. (1965). The Inference to the Best Explanation. The Philosophical Review, 74(1), 88–95. Thagard, P. R. (1978). The Best Explanation: Criteria for Theory Choice. The Journal of Philosophy, 75(2), 76–92.
    AbductiveIbePaper,
    "cognition/abductive-ibe",
    WitnessFamily::Paper,
    "AbductiveIbe",
    None
);

witness_marker!(
    /// Kakas, A. C., Kowalski, R. A., & Toni, F. (1992). Abductive Logic Programming. Journal of Logic and Computation, 2(6), 719–770.
    AbductiveLpPaper,
    "cognition/abductive-lp",
    WitnessFamily::Paper,
    "AbductiveLp",
    None
);

witness_marker!(
    /// Anderson, J. R., & Lebiere, C. (1998). Lawrence Erlbaum Associates. Chapter 3 (the activation equation A_i = B_i + Σ_j W_j·S_ji) and Chapter 9 (cognitive arithmetic: retrieval of addition facts).
    ActRPaper,
    "cognition/act-r",
    WitnessFamily::Paper,
    "ActR",
    None
);

witness_marker!(
    /// Allen, J. F. (1983). Maintaining Knowledge about Temporal Intervals. Communications of the ACM, 26(11), 832-843.
    AllenTemporalPaper,
    "cognition/allen-temporal",
    WitnessFamily::Paper,
    "AllenTemporal",
    None
);

witness_marker!(
    /// Falkenhainer, B., Forbus, K. D., & Gentner, D. (1989). Artificial Intelligence, 41(1), 1-63.
    AnalogySmePaper,
    "cognition/analogy-sme",
    WitnessFamily::Paper,
    "AnalogySme",
    None
);

witness_marker!(
    /// Gelfond, M., & Lifschitz, V. (1988). The Stable Model Semantics for Logic Programming. In: Proceedings of the Fifth International Conference and Symposium on Logic Programming (ICLP/SLP 1988), MIT Press, pp. 1070–1080.
    AspPaper,
    "cognition/asp",
    WitnessFamily::Paper,
    "Asp",
    None
);

witness_marker!(
    /// Sussman, Gerald Jay. "A Computational Model of Skill Acquisition." MIT Artificial Intelligence Laboratory Technical Report AI TR-297, August 1973. Thesis Supervisor: Seymour A. Papert.
    AutoinstinctLearningPaper,
    "cognition/autoinstinct-learning",
    WitnessFamily::Paper,
    "AutoinstinctLearning",
    None
);

witness_marker!(
    /// Boden, M. A. (1977). Artificial Intelligence and Natural Man. New York: Basic Books. Chapter 6: "Programs That Model the Mind." [Conflict detection framework: neurosis as simultaneous maintenance of mutually incompatible beliefs, pp. 198–212.]
    AutoinstinctNeurosisPaper,
    "cognition/autoinstinct-neurosis",
    WitnessFamily::Paper,
    "AutoinstinctNeurosis",
    None
);

witness_marker!(
    /// Schank, R. C. (1972). Conceptual dependency: A theory of natural language understanding. Cognitive Psychology, 3(4), 552–631. https://doi.org/10.1016/0010-0285(72)90022-9
    AutoinstinctSemanticsPaper,
    "cognition/autoinstinct-semantics",
    WitnessFamily::Paper,
    "AutoinstinctSemantics",
    None
);

witness_marker!(
    /// Marr, D., & Poggio, T. (1976). Cooperative computation of stereo disparity. Science, 194(4262), 283–287. https://doi.org/10.1126/science.968816
    AutoinstinctVisionPaper,
    "cognition/autoinstinct-vision",
    WitnessFamily::Paper,
    "AutoinstinctVision",
    None
);

witness_marker!(
    /// Pearl, J. (1988). Probabilistic Reasoning in Intelligent Systems: Networks of Plausible Inference. Morgan Kaufmann.
    BayesianNetworkPaper,
    "cognition/bayesian-network",
    WitnessFamily::Paper,
    "BayesianNetwork",
    None
);

witness_marker!(
    /// Konieczny, S., & Pino Pérez, R. (2002). Merging Information Under Constraints: A Logical Framework. Journal of Logic and Computation, 12(5), 773–808.
    BeliefMergingPaper,
    "cognition/belief-merging",
    WitnessFamily::Paper,
    "BeliefMerging",
    None
);

witness_marker!(
    /// A. Aamodt, E. Plaza (1994). Case-Based Reasoning: Foundational Issues, Methodological Variations, and System Approaches. AI Communications, IOS Press, Vol. 7, No. 1, pp. 39–59.
    CbrPaper,
    "cognition/cbr",
    WitnessFamily::Paper,
    "Cbr",
    None
);

witness_marker!(
    /// McCarthy, J. (1980). Artificial Intelligence, 13(1-2), 27-39.
    CircumscriptionPaper,
    "cognition/circumscription",
    WitnessFamily::Paper,
    "Circumscription",
    None
);

witness_marker!(
    /// Jaffar, J., & Lassez, J.-L. (1987). Constraint Logic Programming. In: Proceedings of the 14th ACM SIGACT-SIGPLAN Symposium on Principles of Programming Languages (POPL 1987), pp. 111–119.
    ClpPaper,
    "cognition/clp",
    WitnessFamily::Paper,
    "Clp",
    None
);

witness_marker!(
    /// Adele E. Goldberg (1995). Constructions: A Construction Grammar Approach to Argument Structure. University of Chicago Press.
    ConstructionGrammarPaper,
    "cognition/construction-grammar",
    WitnessFamily::Paper,
    "ConstructionGrammar",
    None
);

witness_marker!(
    /// Stuart Russell and Peter Norvig (2010). Artificial Intelligence: A Modern Approach (3rd ed.). Pearson.
    ContingentPlanPaper,
    "cognition/contingent-plan",
    WitnessFamily::Paper,
    "ContingentPlan",
    None
);

witness_marker!(
    /// Mackworth, A. K. (1977). Consistency in Networks of Relations. Artificial Intelligence, 8(1), 99-118.
    CspAc3Paper,
    "cognition/csp-ac3",
    WitnessFamily::Paper,
    "CspAc3",
    None
);

witness_marker!(
    /// Clarke, E. M., Emerson, E. A., & Sistla, A. P. (1986). ACM Transactions on Programming Languages and Systems, 8(2), 244-263.
    CtlCheckPaper,
    "cognition/ctl-check",
    WitnessFamily::Paper,
    "CtlCheck",
    None
);

witness_marker!(
    /// Reiter, R. (1980). A Logic for Default Reasoning. Artificial Intelligence, 13(1-2), 81-132.
    DefaultLogicPaper,
    "cognition/default-logic",
    WitnessFamily::Paper,
    "DefaultLogic",
    None
);

witness_marker!(
    /// Shafer, G. (1976). A Mathematical Theory of Evidence. Princeton University Press.
    DempsterShaferPaper,
    "cognition/dempster-shafer",
    WitnessFamily::Paper,
    "DempsterShafer",
    None
);

witness_marker!(
    /// Feigenbaum, E. A., Buchanan, B. G., & Lederberg, J. (1971). On generality and problem solving: A case study using the DENDRAL program. Stanford Artificial Intelligence Project Memo AIM-131, Computer Science Department Report CS176. Stanford University.
    DendralPaper,
    "cognition/dendral",
    WitnessFamily::Paper,
    "Dendral",
    None
);

witness_marker!(
    /// Baader, F., Brandt, S., & Lutz, C. (2005). Pushing the EL Envelope. In: Proceedings of the 19th International Joint Conference on Artificial Intelligence (IJCAI 2005), pp. 364–369.
    DescriptionLogicPaper,
    "cognition/description-logic",
    WitnessFamily::Paper,
    "DescriptionLogic",
    None
);

witness_marker!(
    /// Mitchell, T. M., Keller, R. M., & Kedar-Cabelli, S. T. (1986). Explanation-Based Generalization: A Unifying View. Machine Learning, 1(1), 47-80.
    EblPaper,
    "cognition/ebl",
    WitnessFamily::Paper,
    "Ebl",
    None
);

witness_marker!(
    /// Weizenbaum, J. (1966). ELIZA—A computer program for the study of natural language communication between man and machine. Communications of the ACM, 9(1), 36–45. https://doi.org/10.1145/365153.365168
    ElizaPaper,
    "cognition/eliza",
    WitnessFamily::Paper,
    "Eliza",
    None
);

witness_marker!(
    /// Tulving, E. (1983). Oxford University Press (Ch. 7: encoding specificity and temporal organisation). Nuxoll, A. M., & Laird, J. E. (2007). Proceedings of AAAI 2007, 1560-1565 (cue-based retrieval of the best partially-matching episode).
    EpisodicMemoryPaper,
    "cognition/episodic-memory",
    WitnessFamily::Paper,
    "EpisodicMemory",
    None
);

witness_marker!(
    /// Kowalski, R., & Sergot, M. (1986). A Logic-based Calculus of Events. New Generation Computing, 4(1), 67–95.
    EventCalculusPaper,
    "cognition/event-calculus",
    WitnessFamily::Paper,
    "EventCalculus",
    None
);

witness_marker!(
    /// Minsky, M. (1974). A Framework for Representing Knowledge. MIT AI Laboratory Memo 306. Reprinted in Winston (Ed.), The Psychology of Computer Vision, 1975.
    FramesInheritancePaper,
    "cognition/frames-inheritance",
    WitnessFamily::Paper,
    "FramesInheritance",
    None
);

witness_marker!(
    /// Mamdani, E. H., & Assilian, S. (1975). An Experiment in Linguistic Synthesis with a Fuzzy Logic Controller. International Journal of Man-Machine Studies, 7(1), 1-13.
    FuzzyLogicPaper,
    "cognition/fuzzy-logic",
    WitnessFamily::Paper,
    "FuzzyLogic",
    None
);

witness_marker!(
    /// A. Newell and H. A. Simon, "GPS, A Program That Simulates Human Thought," RAND Corporation, P-2257, March 30, 1961 (Revised April 10, 1961). Presented at the Conference on Learning Automata, Karlsruhe, Germany, April 11–14, 1961.
    GpsPaper,
    "cognition/gps",
    WitnessFamily::Paper,
    "Gps",
    None
);

witness_marker!(
    /// Erman, L. D., Hayes-Roth, F., Lesser, V. R., and Reddy, D. R. (1980). The Hearsay-II speech-understanding system: Integrating knowledge to resolve uncertainty. ACM Computing Surveys, 12(2), 213–253.
    HearsayPaper,
    "cognition/hearsay",
    WitnessFamily::Paper,
    "Hearsay",
    None
);

witness_marker!(
    /// Nau, D., Au, T.-C., Ilghami, O., Kuter, U., Murdock, J. W., Wu, D., & Yaman, F. (2003). SHOP2: An HTN Planning System. Journal of Artificial Intelligence Research, 20, 379-404.
    HtnPlanningPaper,
    "cognition/htn-planning",
    WitnessFamily::Paper,
    "HtnPlanning",
    None
);

witness_marker!(
    /// Quinlan, J. R. (1990). Machine Learning, 5(3), 239-266.
    IlpPaper,
    "cognition/ilp",
    WitnessFamily::Paper,
    "Ilp",
    None
);

witness_marker!(
    /// Havelund, K., & Rosu, G. (2001). Monitoring Programs Using Rewriting. Proceedings of ASE 2001 (16th IEEE International Conference on Automated Software Engineering), 135-143.
    LtlMonitorPaper,
    "cognition/ltl-monitor",
    WitnessFamily::Paper,
    "LtlMonitor",
    None
);

witness_marker!(
    /// Matthew Richardson and Pedro Domingos (2006). Markov logic networks. Machine Learning 62(1-2):107-136.
    MarkovLogicPaper,
    "cognition/markov-logic",
    WitnessFamily::Paper,
    "MarkovLogic",
    None
);

witness_marker!(
    /// Bellman, R. (1957). Dynamic Programming. Princeton University Press. (Functional equation / principle of optimality, Chapter III; successive-approximation solution, Chapter IV.)
    MdpPaper,
    "cognition/mdp",
    WitnessFamily::Paper,
    "Mdp",
    None
);

witness_marker!(
    /// Michael T. Cox and Anita Raja (eds.) (2011). Metareasoning: Thinking about Thinking. MIT Press.
    MetaReasoningPaper,
    "cognition/meta-reasoning",
    WitnessFamily::Paper,
    "MetaReasoning",
    None
);

witness_marker!(
    /// Zwicky, F. (1969). Discovery, Invention, Research Through the Morphological Approach. Macmillan.
    MorphologicalPaper,
    "cognition/morphological",
    WitnessFamily::Paper,
    "Morphological",
    None
);

witness_marker!(
    /// Shortliffe, E. H., & Buchanan, B. G. (1975). A model of inexact reasoning in medicine. Mathematical Biosciences, 23(3–4), 351–379. Reprinted as Chapter 11 in: Buchanan, B. G., & Shortliffe, E. H. (Eds.), Rule-Based Expert Systems. Addison-Wesley, 1984.
    MycinPaper,
    "cognition/mycin",
    WitnessFamily::Paper,
    "Mycin",
    None
);

witness_marker!(
    /// Hayes, P. J. (1979). In D. Michie (Ed.), Expert Systems in the Micro-Electronic Age. Edinburgh University Press. Hayes, P. J. (1985). In J. R. Hobbs & R. C. Moore (Eds.), Formal Theories of the Commonsense World (pp. 71-107). Ablex.
    NaivePhysicsPaper,
    "cognition/naive-physics",
    WitnessFamily::Paper,
    "NaivePhysics",
    None
);

witness_marker!(
    /// van der Aalst, W.M.P. (2019). Object-Centric Process Mining: Dealing with Divergence and Convergence.
    OcpmRouteDiscovererPaper,
    "cognition/ocpm-route-discoverer",
    WitnessFamily::Paper,
    "OcpmRouteDiscoverer",
    None
);

witness_marker!(
    /// McAllester, D., & Rosenblitt, D. (1991). Systematic Nonlinear Planning. In: Proceedings of the Ninth National Conference on Artificial Intelligence (AAAI-91), pp. 634–639.
    PartialOrderPlanPaper,
    "cognition/partial-order-plan",
    WitnessFamily::Paper,
    "PartialOrderPlan",
    None
);

witness_marker!(
    /// Leslie Pack Kaelbling, Michael L. Littman and Anthony R. Cassandra (1998). Planning and acting in partially observable stochastic domains. Artificial Intelligence 101(1-2):99-134.
    PomdpPaper,
    "cognition/pomdp",
    WitnessFamily::Paper,
    "Pomdp",
    None
);

witness_marker!(
    /// De Raedt, L., Kimmig, A., & Toivonen, H. (2007). Proceedings of IJCAI 2007, 2468-2473.
    ProblogPaper,
    "cognition/problog",
    WitnessFamily::Paper,
    "Problog",
    None
);

witness_marker!(
    /// Kowalski, R. A. (1974). Predicate logic as programming language. IFIP Congress 74, Stockholm, pp. 569–574.
    PrologPaper,
    "cognition/prolog",
    WitnessFamily::Paper,
    "Prolog",
    None
);

witness_marker!(
    /// de Kleer, J., & Brown, J. S. (1984). A Qualitative Physics Based on Confluences. Artificial Intelligence, 24(1–3), 7–83.
    QualitativeReasonPaper,
    "cognition/qualitative-reason",
    WitnessFamily::Paper,
    "QualitativeReason",
    None
);

witness_marker!(
    /// Watkins, C. J. C. H., & Dayan, P. (1992). Machine Learning, 8(3-4), 279-292.
    RlSymbolicPaper,
    "cognition/rl-symbolic",
    WitnessFamily::Paper,
    "RlSymbolic",
    None
);

witness_marker!(
    /// Marques-Silva, J. P., & Sakallah, K. A. (1999). IEEE Transactions on Computers, 48(5), 506-521.
    SatCdclPaper,
    "cognition/sat-cdcl",
    WitnessFamily::Paper,
    "SatCdcl",
    None
);

witness_marker!(
    /// Schank, R. C., & Abelson, R. P. (1977). Scripts, Plans, Goals and Understanding: An Inquiry into Human Knowledge Structures. Lawrence Erlbaum Associates.
    ScriptSamPaper,
    "cognition/script-sam",
    WitnessFamily::Paper,
    "ScriptSam",
    None
);

witness_marker!(
    /// Reiter, R. (1991). In V. Lifschitz (Ed.), Artificial Intelligence and Mathematical Theory of Computation: Papers in Honor of John McCarthy (pp. 359-380). Academic Press.
    SituationCalculusPaper,
    "cognition/situation-calculus",
    WitnessFamily::Paper,
    "SituationCalculus",
    None
);

witness_marker!(
    /// Laird, J. E., Newell, A., & Rosenbloom, P. S. (1987). Soar: An architecture for general intelligence. Artificial Intelligence, 33(1), 1–64. https://doi.org/10.1016/0004-3702(87)90050-6
    SoarPaper,
    "cognition/soar",
    WitnessFamily::Paper,
    "Soar",
    None
);

witness_marker!(
    /// Fikes, R. E., & Nilsson, N. J. (1971). STRIPS: A new approach to the application of theorem proving to problem solving. Artificial Intelligence, 2(3–4), 189–208.
    StripsPaper,
    "cognition/strips",
    WitnessFamily::Paper,
    "Strips",
    None
);

witness_marker!(
    /// Raymond M. Smullyan (1968). First-Order Logic. Springer-Verlag, Ergebnisse der Mathematik und ihrer Grenzgebiete 43.
    TableauxPaper,
    "cognition/tableaux",
    WitnessFamily::Paper,
    "Tableaux",
    None
);

witness_marker!(
    /// Altshuller, G. (1984). Creativity as an Exact Science. Gordon and Breach Science Publishers.
    TrizPaper,
    "cognition/triz",
    WitnessFamily::Paper,
    "Triz",
    None
);

witness_marker!(
    /// Mitchell, T. M. (1982). Generalization as Search. Artificial Intelligence, 18(2), 203–226.
    VersionSpacePaper,
    "cognition/version-space",
    WitnessFamily::Paper,
    "VersionSpace",
    None
);

