use crate::prelude::*;

use super::ownership_info::OwnershipInfo;
use super::product::Product;

/// Products owned by the organization or person.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum owns {
    OwnershipInfo(OwnershipInfo),
    Product(Product),
}
