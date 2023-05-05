use crate::prelude::*;

use super::administrative_area::AdministrativeArea;
use super::geo_shape::GeoShape;
use super::place::Place;
use super::text::Text;


/// http://schema.org/areaServed
/// The geographic area where a service or offered item is provided.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum AreaServedPropEnum {
    AdministrativeArea(AdministrativeArea),
    GeoShape(GeoShape),
    Place(Place),
    Text(Text),
}
