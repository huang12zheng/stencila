use crate::prelude::*;

use super::place::Place;
use super::postal_address::PostalAddress;
use super::url::URL;

/// Real or fictional location of the game (or part of game).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum gameLocation {
    Place(Place),
    PostalAddress(PostalAddress),
    URL(URL),
}
