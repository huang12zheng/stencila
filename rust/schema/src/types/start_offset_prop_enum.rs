use crate::prelude::*;

use super::hyper_toc_entry::HyperTocEntry;
use super::number::Number;


/// http://schema.org/startOffset
/// The start time of the clip expressed as the number of seconds from the beginning of the work.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum StartOffsetPropEnum {
    HyperTocEntry(HyperTocEntry),
    Number(Number),
}
