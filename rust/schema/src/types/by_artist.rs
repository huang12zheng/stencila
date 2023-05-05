use crate::prelude::*;

use super::music_group::MusicGroup;
use super::person::Person;

/// The artist that performed this album or recording.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum byArtist {
    MusicGroup(MusicGroup),
    Person(Person),
}
