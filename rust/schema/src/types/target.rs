use crate::prelude::*;

use super::entry_point::EntryPoint;
use super::url::URL;

/// Indicates a target EntryPoint, or url, for an Action.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum target {
    EntryPoint(EntryPoint),
    URL(URL),
}
