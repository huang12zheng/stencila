use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;
use super::place::Place;

/// The <a href="http://www.gs1.org/gln">Global Location Number</a> (GLN, sometimes also referred to as International Location Number or ILN) of the respective organization, person, or place. The GLN is a 13-digit number used to identify parties and physical locations.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum globalLocationNumber {
    Organization(Organization),
    Person(Person),
    Place(Place),
}
