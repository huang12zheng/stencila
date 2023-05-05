use crate::prelude::*;

use super::defined_region::DefinedRegion;
use super::postal_address::PostalAddress;

/// The region in which the locality is, and which is in the country. For example, California or another appropriate first-level <a href="https://en.wikipedia.org/wiki/List_of_administrative_divisions_by_country">Administrative division</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum addressRegion {
    DefinedRegion(DefinedRegion),
    PostalAddress(PostalAddress),
}
