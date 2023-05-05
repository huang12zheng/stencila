use crate::prelude::*;

use super::geospatial_geometry::GeospatialGeometry;
use super::place::Place;


/// http://schema.org/geoWithin
/// Represents a relationship between two geometries (or the places they represent), relating a geometry to one that contains it, i.e. it is inside (i.e. within) its interior. As defined in <a href="https://en.wikipedia.org/wiki/DE-9IM">DE-9IM</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum GeoWithinPropEnum {
    GeospatialGeometry(GeospatialGeometry),
    Place(Place),
}
