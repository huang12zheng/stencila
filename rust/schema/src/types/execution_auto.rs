use crate::prelude::*;

/// Under which circumstances the document node should be automatically executed.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Defaults, Read, Write)]
#[serde(untagged, crate = "common::serde")]
#[def = "Needed"]
pub enum ExecutionAuto {
    Never,
    Needed,
    Always,
}
