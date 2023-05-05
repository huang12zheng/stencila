use crate::prelude::*;

use super::order_item::OrderItem;
use super::product::Product;
use super::service::Service;

/// The item ordered.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum orderedItem {
    OrderItem(OrderItem),
    Product(Product),
    Service(Service),
}
