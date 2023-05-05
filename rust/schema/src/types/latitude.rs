use crate::prelude::*;

use super::geo_coordinates::GeoCoordinates;
use super::place::Place;

/// The latitude of a location. For example <code>37.42242</code> (<a href="https://en.wikipedia.org/wiki/World_Geodetic_System">WGS 84</a>).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum latitude {
    GeoCoordinates(GeoCoordinates),
    Place(Place),
}
