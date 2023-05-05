use crate::prelude::*;

use super::place::Place;
use super::postal_address::PostalAddress;
use super::text::Text;
use super::virtual_location::VirtualLocation;

/// The location of, for example, where an event is happening, where an organization is located, or where an action takes place.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum location {
    Place(Place),
    PostalAddress(PostalAddress),
    Text(Text),
    VirtualLocation(VirtualLocation),
}
