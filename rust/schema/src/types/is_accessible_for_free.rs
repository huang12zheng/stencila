use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::event::Event;
use super::place::Place;

/// A flag to signal that the item, event, or place is accessible for free.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum isAccessibleForFree {
    CreativeWork(CreativeWork),
    Event(Event),
    Place(Place),
}
