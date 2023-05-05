use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::event::Event;

/// A CreativeWork or Event about this Thing.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum subjectOf {
    CreativeWork(CreativeWork),
    Event(Event),
}
