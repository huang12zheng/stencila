use crate::prelude::*;

use super::invoice::Invoice;
use super::order::Order;

/// A number that confirms the given order or payment has been received.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum confirmationNumber {
    Invoice(Invoice),
    Order(Order),
}
