use crate::prelude::*;

use super::music_group::MusicGroup;
use super::person::Person;


/// http://schema.org/musicBy
/// The composer of the soundtrack.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum MusicByPropEnum {
    MusicGroup(MusicGroup),
    Person(Person),
}
