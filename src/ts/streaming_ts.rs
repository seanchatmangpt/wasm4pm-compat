use serde::{Deserialize, Serialize};
use specta::Type;
use tsify::Tsify;

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct EventWindowTs {
    pub events: Vec<String>,
    pub size: usize,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum StreamingRefusalTs {
    WindowOverflow,
    OutOfOrderArrival,
    SourceDisconnected,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum TemporalOrderTs {
    Before,
    After,
    Concurrent,
    Unknown,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct TemporalProfileTs {
    pub relations: Vec<(String, String, TemporalOrderTs)>,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum TemporalRefusalTs {
    ClockDriftDetected,
    NonMonotonicTimestamps,
}
