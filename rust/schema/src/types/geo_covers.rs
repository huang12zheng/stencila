use crate::prelude::*;

use super::geospatial_geometry::GeospatialGeometry;
use super::place::Place;

/// Represents a relationship between two geometries (or the places they represent), relating a covering geometry to a covered geometry. "Every point of b is a point of (the interior or boundary of) a". As defined in <a href="https://en.wikipedia.org/wiki/DE-9IM">DE-9IM</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum geoCovers {
    GeospatialGeometry(GeospatialGeometry),
    Place(Place),
}
