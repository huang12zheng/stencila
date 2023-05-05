use crate::prelude::*;

use super::invoice::Invoice;
use super::order::Order;

/// Party placing the order or paying the invoice.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum customer {
    Invoice(Invoice),
    Order(Order),
}
