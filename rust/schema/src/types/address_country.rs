use crate::prelude::*;

use super::defined_region::DefinedRegion;
use super::geo_coordinates::GeoCoordinates;
use super::geo_shape::GeoShape;
use super::postal_address::PostalAddress;

/// The country. For example, USA. You can also provide the two-letter <a href="http://en.wikipedia.org/wiki/ISO_3166-1">ISO 3166-1 alpha-2 country code</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum addressCountry {
    DefinedRegion(DefinedRegion),
    GeoCoordinates(GeoCoordinates),
    GeoShape(GeoShape),
    PostalAddress(PostalAddress),
}
