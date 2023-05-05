use crate::prelude::*;

use super::quantitative_value::QuantitativeValue;
use super::text::Text;


/// http://schema.org/intensity
/// Quantitative measure gauging the degree of force involved in the exercise, for example, heartbeats per minute. May include the velocity of the movement.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum IntensityPropEnum {
    QuantitativeValue(QuantitativeValue),
    Text(Text),
}
