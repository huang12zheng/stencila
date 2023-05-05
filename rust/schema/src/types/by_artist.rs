use crate::prelude::*;

use super::music_album::MusicAlbum;
use super::music_recording::MusicRecording;

/// The artist that performed this album or recording.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum byArtist {
    MusicAlbum(MusicAlbum),
    MusicRecording(MusicRecording),
}
