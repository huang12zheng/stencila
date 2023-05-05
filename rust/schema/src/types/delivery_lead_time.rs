use crate::prelude::*;

use super::demand::Demand;
use super::offer::Offer;

/// The typical delay between the receipt of the order and the goods either leaving the warehouse or being prepared for pickup, in case the delivery method is on site pickup.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum deliveryLeadTime {
    Demand(Demand),
    Offer(Offer),
}
