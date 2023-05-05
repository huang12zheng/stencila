use crate::prelude::*;

use super::order::Order;
use super::order_item::OrderItem;

/// The delivery of the parcel related to this order or order item.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum orderDelivery {
    Order(Order),
    OrderItem(OrderItem),
}
