use crate::prelude::*;

use super::person::Person;
use super::sports_team::SportsTeam;


/// http://schema.org/competitor
/// A competitor in a sports event.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum CompetitorPropEnum {
    Person(Person),
    SportsTeam(SportsTeam),
}
