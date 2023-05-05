use crate::prelude::*;

use super::place::Place;
use super::postal_address::PostalAddress;
use super::text::Text;


/// http://schema.org/itemLocation
/// Current location of the item.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ItemLocationPropEnum {
    Place(Place),
    PostalAddress(PostalAddress),
    Text(Text),
}
