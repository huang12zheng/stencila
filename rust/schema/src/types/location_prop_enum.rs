use crate::prelude::*;

use super::place::Place;
use super::postal_address::PostalAddress;
use super::text::Text;
use super::virtual_location::VirtualLocation;


/// http://schema.org/location
/// The location of, for example, where an event is happening, where an organization is located, or where an action takes place.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum LocationPropEnum {
    Place(Place),
    PostalAddress(PostalAddress),
    Text(Text),
    VirtualLocation(VirtualLocation),
}
