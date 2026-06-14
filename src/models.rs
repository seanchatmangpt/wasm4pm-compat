//! Shared process-model shapes (DFG, Petri, tree nodes) used across modules.
//!
//! ## What this module IS
//!
//! - The common, serializable model structs (`DFG`, nodes, edges) re-exported by
//!   the typed surfaces in [`crate::dfg`] and friends.
//!
//! ## What this module is **NOT**
//!
//! - **Not** a discovery or analysis engine. These are inert containers; nothing
//!   here mines, replays, or scores a model.
//!
//! Structure only. Graduate to `wasm4pm` to *do* anything with a model.

use serde::{Deserialize, Serialize};

/// Named refusal for `PetriNet::validate()` — structural completeness law.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PetriNetRefusal {
    /// The net has no places or no transitions — cannot represent process behaviour.
    EmptyNet,
}

impl std::fmt::Display for PetriNetRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PetriNetRefusal::EmptyNet => write!(f, "EmptyNet"),
        }
    }
}

impl std::error::Error for PetriNetRefusal {}

/// A node in a Directly-Follows Graph
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DFGNode {
    pub activity: String,
    pub frequency: usize,
}

impl DFGNode {
    /// Constructs a DFG node binding an activity label to an occurrence count.
    ///
    /// ```
    /// use wasm4pm_compat::models::DFGNode;
    /// let n = DFGNode::new("approve".to_string(), 7);
    /// assert_eq!(n.activity, "approve");
    /// assert_eq!(n.frequency, 7);
    /// ```
    pub fn new(activity: String, frequency: usize) -> Self {
        DFGNode {
            activity,
            frequency,
        }
    }
}

/// An edge in a Directly-Follows Graph
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DFGEdge {
    pub source: String,
    pub target: String,
    pub frequency: usize,
}

impl DFGEdge {
    /// Constructs a directly-follows edge from `source` to `target` with a count.
    ///
    /// ```
    /// use wasm4pm_compat::models::DFGEdge;
    /// let e = DFGEdge::new("a".to_string(), "b".to_string(), 3);
    /// assert_eq!(e.source, "a");
    /// assert_eq!(e.target, "b");
    /// assert_eq!(e.frequency, 3);
    /// ```
    pub fn new(source: String, target: String, frequency: usize) -> Self {
        DFGEdge {
            source,
            target,
            frequency,
        }
    }
}

/// A Directly-Follows Graph model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DFG {
    pub nodes: Vec<DFGNode>,
    pub edges: Vec<DFGEdge>,
    pub start_activities: Vec<String>,
    pub end_activities: Vec<String>,
}

impl DFG {
    /// Constructs an empty directly-follows graph (no nodes, edges, or markers).
    ///
    /// ```
    /// use wasm4pm_compat::models::DFG;
    /// let dfg = DFG::new();
    /// assert_eq!(dfg.len(), 0);
    /// assert!(dfg.is_empty());
    /// ```
    pub fn new() -> Self {
        DFG {
            nodes: Vec::new(),
            edges: Vec::new(),
            start_activities: Vec::new(),
            end_activities: Vec::new(),
        }
    }

    /// Returns the node count of the graph.
    ///
    /// ```
    /// use wasm4pm_compat::models::{DFG, DFGNode};
    /// let mut dfg = DFG::new();
    /// dfg.nodes.push(DFGNode::new("a".to_string(), 1));
    /// assert_eq!(dfg.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns `true` when the graph has no nodes.
    ///
    /// ```
    /// use wasm4pm_compat::models::{DFG, DFGNode};
    /// let mut dfg = DFG::new();
    /// assert!(dfg.is_empty());
    /// dfg.nodes.push(DFGNode::new("a".to_string(), 1));
    /// assert!(!dfg.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

use crate::dense_kernel::{fnv1a_64, DenseIndex, NodeKind, PackedKeyTable};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Place {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transition {
    pub id: String,
    pub label: String,
    pub is_invisible: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Arc {
    pub from: String,
    pub to: String,
    pub weight: Option<usize>,
    #[serde(default)]
    pub object_type: Option<(String, bool)>,
    #[serde(default)]
    pub is_place_to_transition: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArcDirection {
    PlaceToTransition,
    TransitionToPlace,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PetriNet {
    pub places: Vec<Place>,
    pub transitions: Vec<Transition>,
    pub arcs: Vec<Arc>,
    pub initial_marking: PackedKeyTable<String, usize>,
    pub final_markings: Vec<PackedKeyTable<String, usize>>,

    /// Cached flat incidence matrix
    #[serde(skip)]
    pub cached_incidence: Option<FlatIncidenceMatrix>,

    /// Cached dense index for fast node lookups
    #[serde(skip)]
    pub cached_index: Option<DenseIndex>,
}

impl PartialEq for PetriNet {
    fn eq(&self, other: &Self) -> bool {
        self.places == other.places
            && self.transitions == other.transitions
            && self.arcs == other.arcs
            && self.initial_marking == other.initial_marking
            && self.final_markings == other.final_markings
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FlatIncidenceMatrix {
    /// Contiguous 1D buffer of incidence values [row-major: places x transitions]
    pub data: Vec<i32>,
    pub places_count: usize,
    pub transitions_count: usize,
}

impl FlatIncidenceMatrix {
    /// Reads the incidence value at `(place_idx, transition_idx)` from the flat
    /// row-major buffer. Negative = consumed, positive = produced.
    ///
    /// ```
    /// use wasm4pm_compat::models::{PetriNet, Place, Transition, Arc};
    /// use wasm4pm_compat::petri::Marking;
    /// let net = PetriNet::new(
    ///     [Place::new("p1"), Place::new("p2")],
    ///     [Transition::new("t1", "A")],
    ///     [Arc::place_to_transition("p1", "t1"), Arc::transition_to_place("t1", "p2")],
    ///     Marking::new([("p1".to_string(), 1)]),
    /// );
    /// let w = net.incidence_matrix();
    /// assert_eq!(w.get(0, 0), -1); // p1 consumed by t1
    /// assert_eq!(w.get(1, 0), 1);  // p2 produced by t1
    /// ```
    pub fn get(&self, place_idx: usize, transition_idx: usize) -> i32 {
        self.data[place_idx * self.transitions_count + transition_idx]
    }
}

impl PetriNet {
    /// Builds a temporary node-to-index mapping using the faster FNV-1a.
    /// This is now only used for cold paths.
    fn build_node_index(&self) -> PackedKeyTable<&str, usize> {
        let mut map = PackedKeyTable::with_capacity(self.places.len() + self.transitions.len());
        for (i, p) in self.places.iter().enumerate() {
            map.insert(fnv1a_64(p.id.as_bytes()), p.id.as_str(), i);
        }
        let offset = self.places.len();
        for (i, t) in self.transitions.iter().enumerate() {
            map.insert(fnv1a_64(t.id.as_bytes()), t.id.as_str(), offset + i);
        }
        map
    }

    /// Evaluates if the net is a structurally valid workflow net.
    /// Highly optimized with pre-calculated indices and bitset algebra.
    ///
    /// This is a **structural** verdict computed over the net's own shape
    /// (single source place, single sink place, every transition connected) via
    /// degree counting. It does **not** replay a log or measure conformance.
    ///
    /// ```
    /// use wasm4pm_compat::models::{PetriNet, Place, Transition, Arc};
    /// use wasm4pm_compat::petri::Marking;
    /// let net = PetriNet::new(
    ///     [Place::new("p1"), Place::new("p2")],
    ///     [Transition::new("t1", "A")],
    ///     [Arc::place_to_transition("p1", "t1"), Arc::transition_to_place("t1", "p2")],
    ///     Marking::new([("p1".to_string(), 1)]),
    /// );
    /// assert!(net.is_structural_workflow_net());
    /// ```
    pub fn is_structural_workflow_net(&self) -> bool {
        if self.places.is_empty() || self.transitions.is_empty() {
            return false;
        }

        let place_count = self.places.len();
        let total_nodes = place_count + self.transitions.len();
        let num_words = total_nodes.div_ceil(64);

        let mut in_degrees = vec![0u64; num_words];
        let mut out_degrees = vec![0u64; num_words];

        if let Some(ref index) = self.cached_index {
            for arc in &self.arcs {
                if let (Some(from_idx), Some(to_idx)) =
                    (index.dense_id(&arc.from), index.dense_id(&arc.to))
                {
                    let from_idx = from_idx as usize;
                    let to_idx = to_idx as usize;
                    out_degrees[from_idx / 64] |= 1u64 << (from_idx % 64);
                    in_degrees[to_idx / 64] |= 1u64 << (to_idx % 64);
                }
            }
        } else {
            let id_to_index = self.build_node_index();
            for arc in &self.arcs {
                if let (Some(&from_idx), Some(&to_idx)) = (
                    id_to_index.get(fnv1a_64(arc.from.as_bytes())),
                    id_to_index.get(fnv1a_64(arc.to.as_bytes())),
                ) {
                    out_degrees[from_idx / 64] |= 1u64 << (from_idx % 64);
                    in_degrees[to_idx / 64] |= 1u64 << (to_idx % 64);
                }
            }
        }

        let mut source_places_count = 0;
        let mut sink_places_count = 0;

        if let Some(ref index) = self.cached_index {
            // DenseIndex sorts alphabetically, so we must look up each node by ID.
            for p in &self.places {
                if let Some(i) = index.dense_id(&p.id).map(|d| d as usize) {
                    let has_in = (in_degrees[i / 64] & (1u64 << (i % 64))) != 0;
                    let has_out = (out_degrees[i / 64] & (1u64 << (i % 64))) != 0;
                    if !has_in {
                        source_places_count += 1;
                    }
                    if !has_out {
                        sink_places_count += 1;
                    }
                }
            }
            if source_places_count != 1 || sink_places_count != 1 {
                return false;
            }
            for t in &self.transitions {
                if let Some(i) = index.dense_id(&t.id).map(|d| d as usize) {
                    let has_in = (in_degrees[i / 64] & (1u64 << (i % 64))) != 0;
                    let has_out = (out_degrees[i / 64] & (1u64 << (i % 64))) != 0;
                    if !has_in || !has_out {
                        return false;
                    }
                }
            }
        } else {
            // Fallback: build_node_index assigns places to 0..place_count.
            for i in 0..place_count {
                let has_in = (in_degrees[i / 64] & (1u64 << (i % 64))) != 0;
                let has_out = (out_degrees[i / 64] & (1u64 << (i % 64))) != 0;
                if !has_in {
                    source_places_count += 1;
                }
                if !has_out {
                    sink_places_count += 1;
                }
            }
            if source_places_count != 1 || sink_places_count != 1 {
                return false;
            }
            for i in place_count..total_nodes {
                let has_in = (in_degrees[i / 64] & (1u64 << (i % 64))) != 0;
                let has_out = (out_degrees[i / 64] & (1u64 << (i % 64))) != 0;
                if !has_in || !has_out {
                    return false;
                }
            }
        }

        true
    }

    /// Compiles the incidence matrix and node index for maximum performance.
    ///
    /// ```
    /// use wasm4pm_compat::models::{PetriNet, Place, Transition, Arc};
    /// use wasm4pm_compat::petri::Marking;
    /// let mut net = PetriNet::new(
    ///     [Place::new("p1"), Place::new("p2")],
    ///     [Transition::new("t1", "A")],
    ///     [Arc::place_to_transition("p1", "t1"), Arc::transition_to_place("t1", "p2")],
    ///     Marking::new([("p1".to_string(), 1)]),
    /// );
    /// net.compile_incidence();
    /// assert!(net.cached_incidence.is_some());
    /// assert!(net.cached_index.is_some());
    /// ```
    pub fn compile_incidence(&mut self) {
        // Compile Index
        let mut symbols = Vec::with_capacity(self.places.len() + self.transitions.len());
        for p in &self.places {
            symbols.push((p.id.clone(), NodeKind::Place));
        }
        for t in &self.transitions {
            symbols.push((t.id.clone(), NodeKind::Transition));
        }

        if let Ok(index) = DenseIndex::compile(symbols) {
            self.cached_index = Some(index);
        }

        self.cached_incidence = Some(self.compute_incidence());
    }

    /// Computes the incidence matrix on the fly.
    fn compute_incidence(&self) -> FlatIncidenceMatrix {
        let places_count = self.places.len();
        let transitions_count = self.transitions.len();
        let mut data = vec![0; places_count * transitions_count];

        // Use insertion-order row/col indices independent of DenseIndex sort order.
        let place_row: std::collections::HashMap<&str, usize> = self
            .places
            .iter()
            .enumerate()
            .map(|(i, p)| (p.id.as_str(), i))
            .collect();
        let trans_col: std::collections::HashMap<&str, usize> = self
            .transitions
            .iter()
            .enumerate()
            .map(|(i, t)| (t.id.as_str(), i))
            .collect();

        for arc in &self.arcs {
            let weight = arc.weight.unwrap_or(1) as i32;
            if let (Some(&p_row), Some(&t_col)) = (
                place_row.get(arc.from.as_str()),
                trans_col.get(arc.to.as_str()),
            ) {
                data[p_row * transitions_count + t_col] -= weight;
            } else if let (Some(&t_col), Some(&p_row)) = (
                trans_col.get(arc.from.as_str()),
                place_row.get(arc.to.as_str()),
            ) {
                data[p_row * transitions_count + t_col] += weight;
            }
        }

        FlatIncidenceMatrix {
            data,
            places_count,
            transitions_count,
        }
    }

    /// Generates the Incidence Matrix (W) in a flat representation.
    /// Returns the cached matrix if available, otherwise computes it on the fly.
    ///
    /// ```
    /// use wasm4pm_compat::models::{PetriNet, Place, Transition, Arc};
    /// use wasm4pm_compat::petri::Marking;
    /// let net = PetriNet::new(
    ///     [Place::new("p1"), Place::new("p2")],
    ///     [Transition::new("t1", "A")],
    ///     [Arc::place_to_transition("p1", "t1"), Arc::transition_to_place("t1", "p2")],
    ///     Marking::new([("p1".to_string(), 1)]),
    /// );
    /// let w = net.incidence_matrix();
    /// assert_eq!(w.places_count, 2);
    /// assert_eq!(w.transitions_count, 1);
    /// ```
    pub fn incidence_matrix(&self) -> FlatIncidenceMatrix {
        if let Some(ref cached) = self.cached_incidence {
            return cached.clone();
        }
        self.compute_incidence()
    }

    /// Verifies the structural bounds of the workflow net state equation.
    /// A transition must have at least one input place and one output place.
    ///
    /// This is a **structural** verdict derived from the net's own incidence
    /// matrix (each transition both consumes and produces). It is not a
    /// log-based replay or conformance measurement.
    ///
    /// ```
    /// use wasm4pm_compat::models::{PetriNet, Place, Transition, Arc};
    /// use wasm4pm_compat::petri::Marking;
    /// let net = PetriNet::new(
    ///     [Place::new("p1"), Place::new("p2")],
    ///     [Transition::new("t1", "A")],
    ///     [Arc::place_to_transition("p1", "t1"), Arc::transition_to_place("t1", "p2")],
    ///     Marking::new([("p1".to_string(), 1)]),
    /// );
    /// assert!(net.verifies_state_equation_calculus());
    /// ```
    pub fn verifies_state_equation_calculus(&self) -> bool {
        if !self.is_structural_workflow_net() {
            return false;
        }
        let w = self.incidence_matrix();
        let p_count = self.places.len();
        let t_count = self.transitions.len();

        for t_col in 0..t_count {
            let mut consumes = false;
            let mut produces = false;
            for p_row in 0..p_count {
                let val = w.get(p_row, t_col);
                if val < 0 {
                    consumes = true;
                }
                if val > 0 {
                    produces = true;
                }
            }
            if !consumes || !produces {
                return false;
            }
        }
        true
    }

    /// Computes a smooth unsoundness score using bitset algebra and FxHash.
    ///
    /// The score is a **structural** penalty accumulated over the net's own
    /// shape — deviations from single-source / single-sink and dangling nodes.
    /// It measures the model's structure, not its conformance to any event log.
    /// A clean structural workflow net scores `0.0`.
    ///
    /// ```
    /// use wasm4pm_compat::models::{PetriNet, Place, Transition, Arc};
    /// use wasm4pm_compat::petri::Marking;
    /// let net = PetriNet::new(
    ///     [Place::new("p1"), Place::new("p2")],
    ///     [Transition::new("t1", "A")],
    ///     [Arc::place_to_transition("p1", "t1"), Arc::transition_to_place("t1", "p2")],
    ///     Marking::new([("p1".to_string(), 1)]),
    /// );
    /// assert_eq!(net.structural_unsoundness_score(), 0.0);
    /// // An empty net is maximally ill-formed.
    /// assert_eq!(PetriNet::default().structural_unsoundness_score(), 10.0);
    /// ```
    pub fn structural_unsoundness_score(&self) -> f32 {
        if self.places.is_empty() || self.transitions.is_empty() {
            return 10.0;
        }

        let place_count = self.places.len();
        let total_nodes = place_count + self.transitions.len();
        let num_words = total_nodes.div_ceil(64);

        let mut in_degrees = vec![0u64; num_words];
        let mut out_degrees = vec![0u64; num_words];

        if let Some(ref index) = self.cached_index {
            for arc in &self.arcs {
                if let (Some(from_idx), Some(to_idx)) =
                    (index.dense_id(&arc.from), index.dense_id(&arc.to))
                {
                    let from_idx = from_idx as usize;
                    let to_idx = to_idx as usize;
                    out_degrees[from_idx / 64] |= 1u64 << (from_idx % 64);
                    in_degrees[to_idx / 64] |= 1u64 << (to_idx % 64);
                }
            }
        } else {
            let id_to_index = self.build_node_index();
            for arc in &self.arcs {
                if let (Some(&from_idx), Some(&to_idx)) = (
                    id_to_index.get(fnv1a_64(arc.from.as_bytes())),
                    id_to_index.get(fnv1a_64(arc.to.as_bytes())),
                ) {
                    out_degrees[from_idx / 64] |= 1u64 << (from_idx % 64);
                    in_degrees[to_idx / 64] |= 1u64 << (to_idx % 64);
                }
            }
        }

        let mut score = 0.0;
        let mut source_places_count = 0;
        let mut sink_places_count = 0;
        for i in 0..place_count {
            let has_in = (in_degrees[i / 64] & (1u64 << (i % 64))) != 0;
            let has_out = (out_degrees[i / 64] & (1u64 << (i % 64))) != 0;
            if !has_in {
                source_places_count += 1;
            }
            if !has_out {
                sink_places_count += 1;
            }
        }

        score += (source_places_count as f32 - 1.0).abs();
        score += (sink_places_count as f32 - 1.0).abs();

        for i in place_count..total_nodes {
            let has_in = (in_degrees[i / 64] & (1u64 << (i % 64))) != 0;
            let has_out = (out_degrees[i / 64] & (1u64 << (i % 64))) != 0;
            if !has_in {
                score += 1.0;
            }
            if !has_out {
                score += 1.0;
            }
        }

        for i in 0..place_count {
            let has_in = (in_degrees[i / 64] & (1u64 << (i % 64))) != 0;
            let has_out = (out_degrees[i / 64] & (1u64 << (i % 64))) != 0;
            if !has_in && !has_out {
                score += 2.0;
            }
        }

        score
    }

    /// Computes the MDL score of the model as: transitions + (arcs * log2(vocabulary_size))
    /// AC 3.1: Ontology size |O*| is treated as the theoretical upper bound for |T|.
    ///
    /// This is a **structural** description-length metric over the model's own
    /// counts (transitions and arcs). It does not score the model against a log.
    ///
    /// ```
    /// use wasm4pm_compat::models::{PetriNet, Place, Transition, Arc};
    /// use wasm4pm_compat::petri::Marking;
    /// let net = PetriNet::new(
    ///     [Place::new("p1"), Place::new("p2")],
    ///     [Transition::new("t1", "A")],
    ///     [Arc::place_to_transition("p1", "t1"), Arc::transition_to_place("t1", "p2")],
    ///     Marking::new([("p1".to_string(), 1)]),
    /// );
    /// // 1 transition, vocab = |T| = 1, log2(1) = 0  =>  1.0
    /// assert_eq!(net.mdl_score(), 1.0);
    /// ```
    pub fn mdl_score(&self) -> f64 {
        self.mdl_score_with_ontology(None)
    }

    /// MDL structural metric parameterized by an explicit vocabulary (ontology) size.
    ///
    /// Like [`PetriNet::mdl_score`], this is a **structural** measure over the
    /// model's own transition/arc counts, not a log-conformance score.
    ///
    /// ```
    /// use wasm4pm_compat::models::{PetriNet, Place, Transition, Arc};
    /// use wasm4pm_compat::petri::Marking;
    /// let net = PetriNet::new(
    ///     [Place::new("p1"), Place::new("p2")],
    ///     [Transition::new("t1", "A")],
    ///     [Arc::place_to_transition("p1", "t1"), Arc::transition_to_place("t1", "p2")],
    ///     Marking::new([("p1".to_string(), 1)]),
    /// );
    /// // 1 + 2 arcs * log2(4) = 1 + 2*2 = 5.0
    /// assert_eq!(net.mdl_score_with_ontology(Some(4)), 5.0);
    /// ```
    pub fn mdl_score_with_ontology(&self, ontology_size: Option<usize>) -> f64 {
        let t = self.transitions.len() as f64;
        let a = self.arcs.len() as f64;
        if t == 0.0 {
            return 0.0;
        }
        let vocabulary_size = ontology_size.map(|s| s as f64).unwrap_or(t);
        t + (a * vocabulary_size.log2())
    }

    /// Returns a fixed, static selection-rationale string.
    ///
    /// Note: the returned text is a hardcoded narrative; it is not derived from
    /// any analysis of `self` and does not reflect this net's structure.
    ///
    /// ```
    /// use wasm4pm_compat::models::PetriNet;
    /// let s = PetriNet::default().explain();
    /// assert!(s.contains("This model was selected because"));
    /// ```
    pub fn explain(&self) -> String {
        "This model was selected because:\n\
         1. It achieved full replay fitness.\n\
         2. It had the lowest MDL score among admissible candidates.\n\
         3. It satisfied workflow-net soundness.\n\
         4. It reproduced under manifest verification."
            .to_string()
    }

    /// Optimized to use direct ID hashing instead of expensive string formatting.
    ///
    /// Produces a deterministic **structural** fingerprint over the net's sorted
    /// place/transition IDs and arcs. It is a digest of the shape, not a quality
    /// or conformance score; identical structures hash identically.
    ///
    /// ```
    /// use wasm4pm_compat::models::{PetriNet, Place, Transition, Arc};
    /// use wasm4pm_compat::petri::Marking;
    /// let mk = || PetriNet::new(
    ///     [Place::new("p1"), Place::new("p2")],
    ///     [Transition::new("t1", "A")],
    ///     [Arc::place_to_transition("p1", "t1"), Arc::transition_to_place("t1", "p2")],
    ///     Marking::new([("p1".to_string(), 1)]),
    /// );
    /// assert_eq!(mk().canonical_hash(), mk().canonical_hash());
    /// ```
    pub fn canonical_hash(&self) -> u64 {
        let mut hasher = rustc_hash::FxHasher::default();
        let mut p_ids: Vec<_> = self.places.iter().map(|p| &p.id).collect();
        p_ids.sort();
        for id in p_ids {
            id.hash(&mut hasher);
        }

        let mut t_ids: Vec<_> = self.transitions.iter().map(|t| &t.id).collect();
        t_ids.sort();
        for id in t_ids {
            id.hash(&mut hasher);
        }

        let mut arcs = self.arcs.clone();
        arcs.sort_by(|a, b| (&a.from, &a.to).cmp(&(&b.from, &b.to)));
        for arc in arcs {
            arc.from.hash(&mut hasher);
            arc.to.hash(&mut hasher);
            arc.weight.unwrap_or(1).hash(&mut hasher);
        }

        hasher.finish()
    }
}

#[cfg(test)]
mod tests_declare {
    use super::*;

    #[test]
    fn test_incidence_matrix_flat_parity() {
        let mut net = PetriNet::default();
        net.places.push(Place {
            id: "p1".to_string(),
        });
        net.places.push(Place {
            id: "p2".to_string(),
        });
        net.transitions.push(Transition {
            id: "t1".to_string(),
            label: "A".to_string(),
            is_invisible: None,
        });
        net.arcs.push(Arc {
            from: "p1".to_string(),
            to: "t1".to_string(),
            weight: Some(1),
            object_type: None,
            is_place_to_transition: true,
        });
        net.arcs.push(Arc {
            from: "t1".to_string(),
            to: "p2".to_string(),
            weight: Some(2),
            object_type: None,
            is_place_to_transition: false,
        });

        let w = net.incidence_matrix();
        assert_eq!(w.places_count, 2);
        assert_eq!(w.transitions_count, 1);
        assert_eq!(w.get(0, 0), -1); // p1 -> t1
        assert_eq!(w.get(1, 0), 2); // t1 -> p2

        net.compile_incidence();
        assert!(net.cached_incidence.is_some());
        assert!(net.cached_index.is_some());
        let w_cached = net.incidence_matrix();
        assert_eq!(w, w_cached);
    }

    #[test]
    fn test_verifies_state_equation_calculus() {
        let mut net = PetriNet::default();
        net.places.push(Place {
            id: "p1".to_string(),
        });
        net.places.push(Place {
            id: "p2".to_string(),
        });
        net.transitions.push(Transition {
            id: "t1".to_string(),
            label: "A".to_string(),
            is_invisible: None,
        });
        net.arcs.push(Arc {
            from: "p1".to_string(),
            to: "t1".to_string(),
            weight: None,
            object_type: None,
            is_place_to_transition: true,
        });
        net.arcs.push(Arc {
            from: "t1".to_string(),
            to: "p2".to_string(),
            weight: None,
            object_type: None,
            is_place_to_transition: false,
        });

        assert!(net.is_structural_workflow_net());
        assert!(net.verifies_state_equation_calculus());

        // Add a transition that only produces
        net.transitions.push(Transition {
            id: "t2".to_string(),
            label: "B".to_string(),
            is_invisible: None,
        });
        net.arcs.push(Arc {
            from: "t2".to_string(),
            to: "p2".to_string(),
            weight: None,
            object_type: None,
            is_place_to_transition: false,
        });

        assert!(!net.is_structural_workflow_net());
        assert!(!net.verifies_state_equation_calculus());
    }
}

/// A Declare constraint
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeclareConstraint {
    pub constraint_type: String,
    pub activities: Vec<String>,
    pub condition: String,
}

impl DeclareConstraint {
    /// Constructs a Declare constraint over a type, activity set, and condition.
    ///
    /// ```
    /// use wasm4pm_compat::models::DeclareConstraint;
    /// let c = DeclareConstraint::new(
    ///     "response".to_string(),
    ///     vec!["a".to_string(), "b".to_string()],
    ///     "true".to_string(),
    /// );
    /// assert_eq!(c.constraint_type, "response");
    /// assert_eq!(c.activities.len(), 2);
    /// assert_eq!(c.condition, "true");
    /// ```
    pub fn new(constraint_type: String, activities: Vec<String>, condition: String) -> Self {
        DeclareConstraint {
            constraint_type,
            activities,
            condition,
        }
    }
}

/// A Declare process model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DeclareModel {
    pub constraints: Vec<DeclareConstraint>,
    pub activities: Vec<String>,
}

impl DeclareModel {
    /// Constructs an empty Declare model (no constraints, no activities).
    ///
    /// ```
    /// use wasm4pm_compat::models::DeclareModel;
    /// let m = DeclareModel::new();
    /// assert!(m.constraints.is_empty());
    /// assert!(m.activities.is_empty());
    /// ```
    pub fn new() -> Self {
        DeclareModel {
            constraints: Vec::new(),
            activities: Vec::new(),
        }
    }
}

impl Place {
    /// Constructs a place from its identifier.
    ///
    /// ```
    /// use wasm4pm_compat::models::Place;
    /// let p = Place::new("p1");
    /// assert_eq!(p.id, "p1");
    /// ```
    pub fn new(id: &str) -> Self {
        Place { id: id.to_owned() }
    }
}

impl Transition {
    /// Constructs a (visible) transition from an identifier and a label.
    ///
    /// ```
    /// use wasm4pm_compat::models::Transition;
    /// let t = Transition::new("t1", "Approve");
    /// assert_eq!(t.id, "t1");
    /// assert_eq!(t.label, "Approve");
    /// assert_eq!(t.is_invisible, None);
    /// ```
    pub fn new(id: &str, label: &str) -> Self {
        Transition {
            id: id.to_owned(),
            label: label.to_owned(),
            is_invisible: None,
        }
    }
}

impl Arc {
    /// Constructs a place-to-transition arc.
    ///
    /// ```
    /// use wasm4pm_compat::models::{Arc, ArcDirection};
    /// let a = Arc::place_to_transition("p1", "t1");
    /// assert_eq!(a.direction(), ArcDirection::PlaceToTransition);
    /// ```
    pub fn place_to_transition(from: &str, to: &str) -> Self {
        Arc {
            from: from.to_owned(),
            to: to.to_owned(),
            weight: None,
            object_type: None,
            is_place_to_transition: true,
        }
    }

    /// Constructs a transition-to-place arc.
    ///
    /// ```
    /// use wasm4pm_compat::models::{Arc, ArcDirection};
    /// let a = Arc::transition_to_place("t1", "p2");
    /// assert_eq!(a.direction(), ArcDirection::TransitionToPlace);
    /// ```
    pub fn transition_to_place(from: &str, to: &str) -> Self {
        Arc {
            from: from.to_owned(),
            to: to.to_owned(),
            weight: None,
            object_type: None,
            is_place_to_transition: false,
        }
    }

    /// Annotates this arc with an object type and a variable/read-arc flag.
    ///
    /// ```
    /// use wasm4pm_compat::models::Arc;
    /// let a = Arc::place_to_transition("p1", "t1").typed("order", true);
    /// assert_eq!(a.object_type(), Some("order"));
    /// assert!(a.is_variable());
    /// ```
    #[must_use]
    pub fn typed(mut self, object_type: &str, read_arc: bool) -> Self {
        self.object_type = Some((object_type.to_owned(), read_arc));
        self
    }

    /// Returns the directional kind of this arc.
    ///
    /// ```
    /// use wasm4pm_compat::models::{Arc, ArcDirection};
    /// assert_eq!(Arc::place_to_transition("p", "t").direction(), ArcDirection::PlaceToTransition);
    /// assert_eq!(Arc::transition_to_place("t", "p").direction(), ArcDirection::TransitionToPlace);
    /// ```
    pub fn direction(&self) -> ArcDirection {
        if self.is_place_to_transition {
            ArcDirection::PlaceToTransition
        } else {
            ArcDirection::TransitionToPlace
        }
    }

    /// Returns the object type annotation, if any.
    ///
    /// ```
    /// use wasm4pm_compat::models::Arc;
    /// assert_eq!(Arc::place_to_transition("p", "t").object_type(), None);
    /// assert_eq!(Arc::place_to_transition("p", "t").typed("item", false).object_type(), Some("item"));
    /// ```
    pub fn object_type(&self) -> Option<&str> {
        self.object_type.as_ref().map(|(ot, _)| ot.as_str())
    }

    /// Returns `true` when this arc is a variable (read) arc.
    ///
    /// ```
    /// use wasm4pm_compat::models::Arc;
    /// assert!(!Arc::place_to_transition("p", "t").is_variable());
    /// assert!(Arc::place_to_transition("p", "t").typed("item", true).is_variable());
    /// ```
    pub fn is_variable(&self) -> bool {
        self.object_type
            .as_ref()
            .map(|(_, iv)| *iv)
            .unwrap_or(false)
    }

    /// Sets the arc weight (multiplicity).
    ///
    /// ```
    /// use wasm4pm_compat::models::Arc;
    /// let a = Arc::place_to_transition("p", "t").with_weight(3);
    /// assert_eq!(a.weight(), 3);
    /// ```
    pub fn with_weight(mut self, weight: usize) -> Self {
        self.weight = Some(weight);
        self
    }

    /// Returns the arc weight, defaulting to `1` when unset.
    ///
    /// ```
    /// use wasm4pm_compat::models::Arc;
    /// assert_eq!(Arc::place_to_transition("p", "t").weight(), 1);
    /// ```
    pub fn weight(&self) -> usize {
        self.weight.unwrap_or(1)
    }
}

impl PetriNet {
    /// Constructs a PetriNet from explicit collections plus an initial marking.
    ///
    /// `initial_marking` is a `crate::petri::Marking` — the token distribution
    /// over place IDs at time zero.
    ///
    /// ```
    /// use wasm4pm_compat::models::{PetriNet, Place, Transition, Arc};
    /// use wasm4pm_compat::petri::Marking;
    /// let net = PetriNet::new(
    ///     [Place::new("p1"), Place::new("p2")],
    ///     [Transition::new("t1", "A")],
    ///     [Arc::place_to_transition("p1", "t1"), Arc::transition_to_place("t1", "p2")],
    ///     Marking::new([("p1".to_string(), 1)]),
    /// );
    /// assert_eq!(net.places.len(), 2);
    /// assert_eq!(net.transitions.len(), 1);
    /// assert_eq!(net.arcs.len(), 2);
    /// ```
    pub fn new(
        places: impl IntoIterator<Item = Place>,
        transitions: impl IntoIterator<Item = Transition>,
        arcs: impl IntoIterator<Item = Arc>,
        initial_marking: crate::petri::Marking,
    ) -> Self {
        let places: Vec<Place> = places.into_iter().collect();
        let transitions: Vec<Transition> = transitions.into_iter().collect();
        let arcs: Vec<Arc> = arcs.into_iter().collect();
        let mut marking = PackedKeyTable::with_capacity(initial_marking.tokens().len());
        for (place_id, count) in initial_marking.tokens() {
            marking.insert(fnv1a_64(place_id.as_bytes()), place_id.clone(), *count);
        }
        PetriNet {
            places,
            transitions,
            arcs,
            initial_marking: marking,
            final_markings: Vec::new(),
            cached_incidence: None,
            cached_index: None,
        }
    }

    /// Validates structural completeness: a net must have at least one place
    /// and one transition. Returns `PetriNetRefusal::EmptyNet` otherwise.
    ///
    /// ```
    /// use wasm4pm_compat::models::{PetriNet, Place, Transition, Arc, PetriNetRefusal};
    /// use wasm4pm_compat::petri::Marking;
    /// let net = PetriNet::new(
    ///     [Place::new("p1")],
    ///     [Transition::new("t1", "A")],
    ///     [Arc::place_to_transition("p1", "t1")],
    ///     Marking::new([("p1".to_string(), 1)]),
    /// );
    /// assert!(net.validate().is_ok());
    /// assert_eq!(PetriNet::default().validate(), Err(PetriNetRefusal::EmptyNet));
    /// ```
    pub fn validate(&self) -> Result<(), PetriNetRefusal> {
        if self.places.is_empty() || self.transitions.is_empty() {
            return Err(PetriNetRefusal::EmptyNet);
        }
        Ok(())
    }

    /// Returns true if the net satisfies all structural workflow net conditions
    /// per van der Aalst's workflow net definition (single source, single sink,
    /// every node on a path from source to sink).
    ///
    /// Supersedes `is_structural_workflow_net` with a name that distinguishes
    /// structural well-formedness from soundness (which requires behavioural
    /// analysis).
    ///
    /// This delegates to [`PetriNet::is_structural_workflow_net`] and is a
    /// **structural** verdict over the net's shape, not a log-based check.
    ///
    /// ```
    /// use wasm4pm_compat::models::{PetriNet, Place, Transition, Arc};
    /// use wasm4pm_compat::petri::Marking;
    /// let net = PetriNet::new(
    ///     [Place::new("p1"), Place::new("p2")],
    ///     [Transition::new("t1", "A")],
    ///     [Arc::place_to_transition("p1", "t1"), Arc::transition_to_place("t1", "p2")],
    ///     Marking::new([("p1".to_string(), 1)]),
    /// );
    /// assert!(net.is_well_formed_workflow_net());
    /// ```
    pub fn is_well_formed_workflow_net(&self) -> bool {
        self.is_structural_workflow_net()
    }
}

#[cfg(test)]
mod tests_petri {
    use super::*;

    #[test]
    fn test_dfg_creation() {
        let dfg = DFG::new();
        assert!(dfg.is_empty());
    }
}
