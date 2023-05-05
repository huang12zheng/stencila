use crate::prelude::*;

use super::hyper_toc_entry::HyperTocEntry;
use super::number::Number;

/// The end time of the clip expressed as the number of seconds from the beginning of the work.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum endOffset {
    HyperTocEntry(HyperTocEntry),
    Number(Number),
}
