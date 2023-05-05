use crate::prelude::*;

use super::geo_coordinates::GeoCoordinates;
use super::geo_shape::GeoShape;

/// The geo coordinates of the place.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum geo {
    GeoCoordinates(GeoCoordinates),
    GeoShape(GeoShape),
}
