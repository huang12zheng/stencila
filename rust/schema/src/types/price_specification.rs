use crate::prelude::*;

use super::demand::Demand;
use super::offer::Offer;
use super::trade_action::TradeAction;

/// One or more detailed price specifications, indicating the unit price and delivery or payment charges.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum priceSpecification {
    Demand(Demand),
    Offer(Offer),
    TradeAction(TradeAction),
}
