use crate::prelude::*;

use super::order::Order;
use super::order_item::OrderItem;

/// The item ordered.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum orderedItem {
    Order(Order),
    OrderItem(OrderItem),
}
