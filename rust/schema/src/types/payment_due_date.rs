use crate::prelude::*;

use super::invoice::Invoice;
use super::order::Order;

/// The date that payment is due.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum paymentDueDate {
    Invoice(Invoice),
    Order(Order),
}
