use crate::prelude::*;

use super::invoice::Invoice;
use super::order::Order;

/// An identifier for the method of payment used (e.g. the last 4 digits of the credit card).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum paymentMethodId {
    Invoice(Invoice),
    Order(Order),
}
