use crate::prelude::*;

use super::quantitative_value::QuantitativeValue;
use super::text::Text;


/// http://schema.org/yield
/// The quantity that results by performing instructions. For example, a paper airplane, 10 personalized candles.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum YieldPropEnum {
    QuantitativeValue(QuantitativeValue),
    Text(Text),
}
