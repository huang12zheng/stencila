use crate::prelude::*;

use super::entry_point::EntryPoint;
use super::url::URL;


/// http://schema.org/target
/// Indicates a target EntryPoint, or url, for an Action.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum TargetPropEnum {
    EntryPoint(EntryPoint),
    URL(URL),
}
