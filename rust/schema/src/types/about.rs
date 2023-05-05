use crate::prelude::*;

use super::communicate_action::CommunicateAction;
use super::creative_work::CreativeWork;
use super::event::Event;

/// The subject matter of the content.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum about {
    CommunicateAction(CommunicateAction),
    CreativeWork(CreativeWork),
    Event(Event),
}
