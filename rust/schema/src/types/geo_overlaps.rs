use crate::prelude::*;

use super::geospatial_geometry::GeospatialGeometry;
use super::place::Place;

/// Represents a relationship between two geometries (or the places they represent), relating a geometry to another that geospatially overlaps it, i.e. they have some but not all points in common. As defined in <a href="https://en.wikipedia.org/wiki/DE-9IM">DE-9IM</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum geoOverlaps {
    GeospatialGeometry(GeospatialGeometry),
    Place(Place),
}
