use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct RuntimeTogglableFeatures {
    pub score_details: bool,
    pub vector_store: bool,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct InstanceTogglableFeatures {
    pub metrics: bool,
}
