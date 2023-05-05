use crate::prelude::*;

use super::audio_object::AudioObject;
use super::clip::Clip;
use super::music_recording::MusicRecording;

/// An embedded audio object.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum audio {
    AudioObject(AudioObject),
    Clip(Clip),
    MusicRecording(MusicRecording),
}
