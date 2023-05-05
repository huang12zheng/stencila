use crate::prelude::*;

use super::geo_coordinates::GeoCoordinates;
use super::geo_shape::GeoShape;

/// The elevation of a location (<a href="https://en.wikipedia.org/wiki/World_Geodetic_System">WGS 84</a>). Values may be of the form 'NUMBER UNIT_OF_MEASUREMENT' (e.g., '1,000 m', '3,200 ft') while numbers alone should be assumed to be a value in meters.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum elevation {
    GeoCoordinates(GeoCoordinates),
    GeoShape(GeoShape),
}
