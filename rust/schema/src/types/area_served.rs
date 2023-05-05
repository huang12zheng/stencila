use crate::prelude::*;

use super::administrative_area::AdministrativeArea;
use super::geo_shape::GeoShape;
use super::place::Place;
use super::text::Text;

/// The geographic area where a service or offered item is provided.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum areaServed {
    AdministrativeArea(AdministrativeArea),
    GeoShape(GeoShape),
    Place(Place),
    Text(Text),
}
