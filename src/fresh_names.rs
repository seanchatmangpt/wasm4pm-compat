//! A8 oracle fresh-name manifest — ggen-rendered from `breed-vocabulary.ttl`.
//!
//! ## What this module IS
//!
//! - A compiled `(breed_id, fresh_name)` table derived from `compat:freshName`
//!   predicates on each `compat:CognitionBreed`. Structure-only lookup data.
//!
//! ## What this module is **NOT**
//!
//! - **Not** hand-authored — edit `breed-vocabulary.ttl` and run
//!   `ggen sync --rule fresh-name-manifest`, never this file.
//! - **Not** an engine. It maps ids to names; it runs nothing.
//!
//! Structure only.

/// The A8 oracle fresh-name table: `(breed_id, fresh_name)` pairs.
///
/// Multiple entries share a `breed_id` when a breed has several oracle names.
/// Consumers filter by `breed_id`:
/// ```
/// use wasm4pm_compat::fresh_names::FRESH_NAME_PAIRS;
/// let names: Vec<&str> = FRESH_NAME_PAIRS
///     .iter()
///     .filter(|(b, _)| *b == "ltl_monitor")
///     .map(|(_, n)| *n)
///     .collect();
/// let _ = names; // structure-only lookup; may be empty if the breed declares none
/// ```
pub const FRESH_NAME_PAIRS: &[(&str, &str)] = &[
    ("abductive_lp", "blarg"),
    ("abductive_lp", "snag"),
    ("allen_temporal", "delta"),
    ("allen_temporal", "eps"),
    ("allen_temporal", "gamma"),
    ("allen_temporal", "pi"),
    ("analogy_sme", "gor"),
    ("analogy_sme", "lum"),
    ("analogy_sme", "rix"),
    ("asp", "blee_atom"),
    ("asp", "zorp_atom"),
    ("bayesian_network", "qchain"),
    ("bayesian_network", "qres"),
    ("bayesian_network", "qubit"),
    ("circumscription", "glows"),
    ("circumscription", "korv"),
    ("csp_ac3", "vblee"),
    ("csp_ac3", "vquux"),
    ("csp_ac3", "vzorp"),
    ("default_logic", "dark_wibble"),
    ("default_logic", "gronk"),
    ("default_logic", "wibble"),
    ("dempster_shafer", "flam"),
    ("dempster_shafer", "flim"),
    ("description_logic", "blurp"),
    ("description_logic", "krumm"),
    ("ebl", "obj2"),
    ("ebl", "obj9"),
    ("frames_inheritance", "snorf"),
    ("frames_inheritance", "welp"),
    ("frames_inheritance", "zilk"),
    ("fuzzy_logic", "flam_var"),
    ("fuzzy_logic", "tri_asymmetric"),
    ("htn_planning", "coach_task"),
    ("htn_planning", "walk_task"),
    ("ltl_monitor", "blee"),
    ("ltl_monitor", "quux"),
    ("ltl_monitor", "zorp"),
    ("meta_reasoning", "breed_blee"),
    ("meta_reasoning", "breed_zorp"),
    ("naive_physics", "bolv"),
    ("naive_physics", "mim"),
    ("naive_physics", "pearl"),
    ("pomdp", "tampered_o"),
    ("problog", "pfact_blee"),
    ("problog", "pfact_quux"),
];
