use crate::prelude::*;

use super::item_list::ItemList;
use super::music_recording::MusicRecording;


/// http://schema.org/track
/// A music recording (track)&#x2014;usually a single song. If an ItemList is given, the list should contain items of type MusicRecording.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum TrackPropEnum {
    ItemList(ItemList),
    MusicRecording(MusicRecording),
}
