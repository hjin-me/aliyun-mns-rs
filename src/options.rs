use serde::{Deserialize, Serialize};
#[derive(Copy, Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ConsumeOptions {
    // WIP
    #[serde(default)]
    pub auto_ack: bool,
}
