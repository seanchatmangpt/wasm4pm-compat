use serde::{Deserialize, Serialize};
use specta::Type;
use tsify::Tsify;

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum ProcessPerspectiveTs {
    ControlFlow,
    Data,
    Resource,
    Time,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum PerspectiveRefusalTs {
    MissingDimension,
    PerspectiveNotSupported,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum CubeDimensionKindTs {
    Activity,
    Resource,
    Time,
    DataAttribute,
    ObjectType,
    CaseAttribute,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct CubeSliceTs {
    pub dimension: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct CubeCellTs {
    pub slices: Vec<CubeSliceTs>,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct ProcessCubeTs {
    pub dimensions: Vec<CubeDimensionKindTs>,
    pub cells: Vec<CubeCellTs>,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum ProcessCubeRefusalTs {
    DimensionMismatch,
    CellUnreachable,
}
