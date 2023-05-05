use crate::prelude::*;

use super::music_group::MusicGroup;
use super::music_playlist::MusicPlaylist;

/// A music recording (track)&#x2014;usually a single song. If an ItemList is given, the list should contain items of type MusicRecording.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum track {
    MusicGroup(MusicGroup),
    MusicPlaylist(MusicPlaylist),
}
