use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::rsvp_action::RsvpAction;

/// Comments, typically from users.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum comment {
    CreativeWork(CreativeWork),
    RsvpAction(RsvpAction),
}
