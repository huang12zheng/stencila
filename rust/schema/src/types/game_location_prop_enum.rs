use crate::prelude::*;

use super::place::Place;
use super::postal_address::PostalAddress;
use super::url::URL;


/// http://schema.org/gameLocation
/// Real or fictional location of the game (or part of game).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum GameLocationPropEnum {
    Place(Place),
    PostalAddress(PostalAddress),
    URL(URL),
}
