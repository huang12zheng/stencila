use crate::prelude::*;

use super::geo_coordinates::GeoCoordinates;
use super::geo_shape::GeoShape;


/// http://schema.org/geo
/// The geo coordinates of the place.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum GeoPropEnum {
    GeoCoordinates(GeoCoordinates),
    GeoShape(GeoShape),
}
