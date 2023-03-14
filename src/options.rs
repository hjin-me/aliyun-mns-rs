use serde::{Deserialize, Serialize};
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ConsumeOptions {
    // #[serde(default)]
    // pub auto_ack: bool,
    #[serde(default)]
    pub prefetch_count: u16,
}

impl Default for ConsumeOptions {
    fn default() -> Self {
        Self {
            // auto_ack: false,
            prefetch_count: 1,
        }
    }
}
