use crate::prelude::*;

use super::monetary_amount::MonetaryAmount;
use super::text::Text;

/// The estimated cost of the supply or supplies consumed when performing instructions.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum estimatedCost {
    MonetaryAmount(MonetaryAmount),
    Text(Text),
}
