use crate::prelude::*;

use super::geo_coordinates::GeoCoordinates;
use super::geo_shape::GeoShape;
use super::organization::Organization;
use super::person::Person;
use super::place::Place;

/// Physical address of the item.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum address {
    GeoCoordinates(GeoCoordinates),
    GeoShape(GeoShape),
    Organization(Organization),
    Person(Person),
    Place(Place),
}
