use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::event::Event;

/// Organization or person who adapts a creative work to different languages, regional differences and technical requirements of a target market, or that translates during some event.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum translator {
    CreativeWork(CreativeWork),
    Event(Event),
}
