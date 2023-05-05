use crate::prelude::*;

use super::ownership_info::OwnershipInfo;
use super::type_and_quantity_node::TypeAndQuantityNode;

/// The product that this structured value is referring to.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum typeOfGood {
    OwnershipInfo(OwnershipInfo),
    TypeAndQuantityNode(TypeAndQuantityNode),
}
