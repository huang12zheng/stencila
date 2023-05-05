use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::event::Event;

/// The typical expected age range, e.g. '7-9', '11-'.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum typicalAgeRange {
    CreativeWork(CreativeWork),
    Event(Event),
}
