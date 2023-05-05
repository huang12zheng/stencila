use crate::prelude::*;

use super::geospatial_geometry::GeospatialGeometry;
use super::place::Place;

/// Represents spatial relations in which two geometries (or the places they represent) touch: "they have at least one boundary point in common, but no interior points." (A symmetric relationship, as defined in <a href="https://en.wikipedia.org/wiki/DE-9IM">DE-9IM</a>.)
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum geoTouches {
    GeospatialGeometry(GeospatialGeometry),
    Place(Place),
}
