use crate::prelude::*;

use super::music_group::MusicGroup;
use super::person::Person;


/// http://schema.org/byArtist
/// The artist that performed this album or recording.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ByArtistPropEnum {
    MusicGroup(MusicGroup),
    Person(Person),
}
