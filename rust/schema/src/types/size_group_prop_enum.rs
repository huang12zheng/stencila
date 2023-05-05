use crate::prelude::*;

use super::size_group_enumeration::SizeGroupEnumeration;
use super::text::Text;


/// http://schema.org/sizeGroup
/// The size group (also known as "size type") for a product's size. Size groups are common in the fashion industry to define size segments and suggested audiences for wearable products. Multiple values can be combined, for example "men's big and tall", "petite maternity" or "regular"
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum SizeGroupPropEnum {
    SizeGroupEnumeration(SizeGroupEnumeration),
    Text(Text),
}
