use crate::prelude::*;

use super::geo_shape::GeoShape;
use super::place::Place;
use super::text::Text;


/// http://schema.org/ineligibleRegion
/// The ISO 3166-1 (ISO 3166-1 alpha-2) or ISO 3166-2 code, the place, or the GeoShape for the geo-political region(s) for which the offer or delivery charge specification is not valid, e.g. a region where the transaction is not allowed.<br/><br/>  See also <a class="localLink" href="/eligibleRegion">eligibleRegion</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum IneligibleRegionPropEnum {
    GeoShape(GeoShape),
    Place(Place),
    Text(Text),
}