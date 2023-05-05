use crate::prelude::*;

use super::geospatial_geometry::GeospatialGeometry;
use super::place::Place;


/// http://schema.org/geoOverlaps
/// Represents a relationship between two geometries (or the places they represent), relating a geometry to another that geospatially overlaps it, i.e. they have some but not all points in common. As defined in <a href="https://en.wikipedia.org/wiki/DE-9IM">DE-9IM</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum GeoOverlapsPropEnum {
    GeospatialGeometry(GeospatialGeometry),
    Place(Place),
}
