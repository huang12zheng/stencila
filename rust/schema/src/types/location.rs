use crate::prelude::*;

use super::action::Action;
use super::event::Event;
use super::interaction_counter::InteractionCounter;
use super::organization::Organization;

/// The location of, for example, where an event is happening, where an organization is located, or where an action takes place.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum location {
    Action(Action),
    Event(Event),
    InteractionCounter(InteractionCounter),
    Organization(Organization),
}
