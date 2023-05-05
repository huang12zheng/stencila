use crate::prelude::*;

use super::demand::Demand;
use super::merchant_return_policy::MerchantReturnPolicy;
use super::offer::Offer;
use super::product::Product;

/// A predefined value from OfferItemCondition specifying the condition of the product or service, or the products or services included in the offer. Also used for product return policies to specify the condition of products accepted for returns.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum itemCondition {
    Demand(Demand),
    MerchantReturnPolicy(MerchantReturnPolicy),
    Offer(Offer),
    Product(Product),
}
