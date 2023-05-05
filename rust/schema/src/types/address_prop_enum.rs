use crate::prelude::*;

use super::postal_address::PostalAddress;
use super::text::Text;


/// http://schema.org/address
/// Physical address of the item.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum AddressPropEnum {
    PostalAddress(PostalAddress),
    Text(Text),
}
