use crate::prelude::*;

use super::audio_object::AudioObject;
use super::clip::Clip;
use super::music_recording::MusicRecording;


/// http://schema.org/audio
/// An embedded audio object.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum AudioPropEnum {
    AudioObject(AudioObject),
    Clip(Clip),
    MusicRecording(MusicRecording),
}
