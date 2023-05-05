use crate::prelude::*;

use super::defined_region::DefinedRegion;
use super::geo_coordinates::GeoCoordinates;
use super::geo_shape::GeoShape;
use super::postal_address::PostalAddress;

/// The postal code. For example, 94043.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum postalCode {
    DefinedRegion(DefinedRegion),
    GeoCoordinates(GeoCoordinates),
    GeoShape(GeoShape),
    PostalAddress(PostalAddress),
}
