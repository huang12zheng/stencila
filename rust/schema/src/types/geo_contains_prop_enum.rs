use crate::prelude::*;

use super::geospatial_geometry::GeospatialGeometry;
use super::place::Place;


/// http://schema.org/geoContains
/// Represents a relationship between two geometries (or the places they represent), relating a containing geometry to a contained geometry. "a contains b iff no points of b lie in the exterior of a, and at least one point of the interior of b lies in the interior of a". As defined in <a href="https://en.wikipedia.org/wiki/DE-9IM">DE-9IM</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum GeoContainsPropEnum {
    GeospatialGeometry(GeospatialGeometry),
    Place(Place),
}
