use crate::prelude::*;

use super::geospatial_geometry::GeospatialGeometry;
use super::place::Place;


/// http://schema.org/geoDisjoint
/// Represents spatial relations in which two geometries (or the places they represent) are topologically disjoint: "they have no point in common. They form a set of disconnected geometries." (A symmetric relationship, as defined in <a href="https://en.wikipedia.org/wiki/DE-9IM">DE-9IM</a>.)
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum GeoDisjointPropEnum {
    GeospatialGeometry(GeospatialGeometry),
    Place(Place),
}
