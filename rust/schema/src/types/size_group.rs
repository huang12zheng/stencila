use crate::prelude::*;

use super::size_group_enumeration::SizeGroupEnumeration;
use super::text::Text;

/// The size group (also known as "size type") for a product's size. Size groups are common in the fashion industry to define size segments and suggested audiences for wearable products. Multiple values can be combined, for example "men's big and tall", "petite maternity" or "regular"
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum sizeGroup {
    SizeGroupEnumeration(SizeGroupEnumeration),
    Text(Text),
}
