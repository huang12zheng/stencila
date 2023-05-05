use crate::prelude::*;

use super::person::Person;
use super::sports_team::SportsTeam;


/// http://schema.org/homeTeam
/// The home team in a sports event.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum HomeTeamPropEnum {
    Person(Person),
    SportsTeam(SportsTeam),
}
