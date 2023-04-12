use crate::prelude::*;

/// Indicates whether the cell is a header or data.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Defaults, Read, Write)]
#[serde(untagged, crate = "common::serde")]
#[def = "Data"]
pub enum TableCellType {
    Data,
    Header,
}
