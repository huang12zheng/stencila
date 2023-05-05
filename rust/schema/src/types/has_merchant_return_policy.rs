use crate::prelude::*;

use super::offer::Offer;
use super::organization::Organization;
use super::product::Product;

/// Specifies a MerchantReturnPolicy that may be applicable.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum hasMerchantReturnPolicy {
    Offer(Offer),
    Organization(Organization),
    Product(Product),
}
