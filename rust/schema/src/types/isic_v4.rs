use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;
use super::place::Place;

/// The International Standard of Industrial Classification of All Economic Activities (ISIC), Revision 4 code for a particular organization, business person, or place.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum isicV4 {
    Organization(Organization),
    Person(Person),
    Place(Place),
}
