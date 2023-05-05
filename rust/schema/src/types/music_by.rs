use crate::prelude::*;

use super::music_group::MusicGroup;
use super::person::Person;

/// The composer of the soundtrack.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum musicBy {
    MusicGroup(MusicGroup),
    Person(Person),
}
