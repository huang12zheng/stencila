use crate::prelude::*;

use super::postal_address::PostalAddress;
use super::text::Text;

/// Physical address of the item.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum address {
    PostalAddress(PostalAddress),
    Text(Text),
}
