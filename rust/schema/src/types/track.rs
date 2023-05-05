use crate::prelude::*;

use super::item_list::ItemList;
use super::music_recording::MusicRecording;

/// A music recording (track)&#x2014;usually a single song. If an ItemList is given, the list should contain items of type MusicRecording.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum track {
    ItemList(ItemList),
    MusicRecording(MusicRecording),
}
