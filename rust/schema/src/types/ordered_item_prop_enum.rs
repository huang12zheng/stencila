use crate::prelude::*;

use super::order_item::OrderItem;
use super::product::Product;
use super::service::Service;


/// http://schema.org/orderedItem
/// The item ordered.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum OrderedItemPropEnum {
    OrderItem(OrderItem),
    Product(Product),
    Service(Service),
}
