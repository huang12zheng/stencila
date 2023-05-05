use crate::prelude::*;

use super::geospatial_geometry::GeospatialGeometry;
use super::place::Place;


/// http://schema.org/geoIntersects
/// Represents spatial relations in which two geometries (or the places they represent) have at least one point in common. As defined in <a href="https://en.wikipedia.org/wiki/DE-9IM">DE-9IM</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum GeoIntersectsPropEnum {
    GeospatialGeometry(GeospatialGeometry),
    Place(Place),
}
