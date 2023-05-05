use crate::prelude::*;

use super::person::Person;
use super::sports_team::SportsTeam;

/// A competitor in a sports event.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum competitor {
    Person(Person),
    SportsTeam(SportsTeam),
}
