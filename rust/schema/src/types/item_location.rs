use crate::prelude::*;

use super::place::Place;
use super::postal_address::PostalAddress;
use super::text::Text;

/// Current location of the item.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum itemLocation {
    Place(Place),
    PostalAddress(PostalAddress),
    Text(Text),
}
