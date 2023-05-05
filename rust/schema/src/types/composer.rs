use crate::prelude::*;

use super::event::Event;
use super::music_composition::MusicComposition;

/// The person or organization who wrote a composition, or who is the composer of a work performed at some event.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum composer {
    Event(Event),
    MusicComposition(MusicComposition),
}
