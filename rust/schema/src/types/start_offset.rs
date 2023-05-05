use crate::prelude::*;

use super::clip::Clip;
use super::seek_to_action::SeekToAction;

/// The start time of the clip expressed as the number of seconds from the beginning of the work.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum startOffset {
    Clip(Clip),
    SeekToAction(SeekToAction),
}
