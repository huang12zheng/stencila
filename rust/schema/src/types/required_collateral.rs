use crate::prelude::*;

use super::text::Text;
use super::thing::Thing;

/// Assets required to secure loan or credit repayments. It may take form of third party pledge, goods, financial instruments (cash, securities, etc.)
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum requiredCollateral {
    Text(Text),
    Thing(Thing),
}
