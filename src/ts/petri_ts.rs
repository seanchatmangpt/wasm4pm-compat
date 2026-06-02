use serde::{Deserialize, Serialize};
use specta::Type;
use tsify::Tsify;

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum ArcDirectionTs {
    PlaceToTransition,
    TransitionToPlace,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct PlaceTs {
    pub id: String,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct TransitionTs {
    pub id: String,
    pub label: String,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct ArcTs {
    pub place_id: String,
    pub transition_id: String,
    pub direction: ArcDirectionTs,
    pub weight: u32,
    pub object_type: Option<String>,
    pub variable: bool,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct MarkingTs {
    pub tokens: Vec<(String, u32)>,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct InitialFinalMarkingPairTs {
    pub initial: MarkingTs,
    pub final_marking: MarkingTs,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct PetriNetTs {
    pub places: Vec<PlaceTs>,
    pub transitions: Vec<TransitionTs>,
    pub arcs: Vec<ArcTs>,
    pub initial: MarkingTs,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum PetriRefusalTs {
    MissingInitialMarking,
    MissingFinalMarking,
    DeadTransition,
    UnsafeNet,
    UnboundedNet,
    ObjectTypeNotPreserved,
    InvalidVariableArc,
    SoundnessNotWitnessed,
    InvalidCancellationRegion,
    InvalidInstanceBounds,
}
