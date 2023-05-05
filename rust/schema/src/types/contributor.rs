use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::event::Event;

/// A secondary contributor to the CreativeWork or Event.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum contributor {
    CreativeWork(CreativeWork),
    Event(Event),
}
