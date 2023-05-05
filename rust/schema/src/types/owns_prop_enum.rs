use crate::prelude::*;

use super::ownership_info::OwnershipInfo;
use super::product::Product;


/// http://schema.org/owns
/// Products owned by the organization or person.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum OwnsPropEnum {
    OwnershipInfo(OwnershipInfo),
    Product(Product),
}
