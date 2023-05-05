use crate::prelude::*;

use super::event::Event;
use super::place::Place;

/// The total number of individuals that may attend an event or venue.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum maximumAttendeeCapacity {
    Event(Event),
    Place(Place),
}
