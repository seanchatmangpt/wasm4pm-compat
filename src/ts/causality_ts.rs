use serde::{Deserialize, Serialize};
use specta::Type;
use tsify::Tsify;

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct CausalLinkTs {
    pub from_event_id: String,
    pub to_event_id: String,
    pub qualifier: Option<String>,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct CausalChainTs {
    pub links: Vec<CausalLinkTs>,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum CausalConsistencyTs {
    Consistent,
    HasCycles,
    HasContradictions,
    Unknown,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum CorrelationSchemaTs {
    ByCase,
    ByObject,
    ByTimestamp,
    ByAttribute,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct CorrelationKeyTs {
    pub schema: CorrelationSchemaTs,
    pub attribute_name: Option<String>,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct CorrelatedLogTs {
    pub correlation_key: CorrelationKeyTs,
    pub matched_event_pairs: Vec<(String, String)>,
}
