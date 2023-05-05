use crate::prelude::*;

use super::monetary_amount::MonetaryAmount;
use super::text::Text;


/// http://schema.org/estimatedCost
/// The estimated cost of the supply or supplies consumed when performing instructions.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum EstimatedCostPropEnum {
    MonetaryAmount(MonetaryAmount),
    Text(Text),
}
