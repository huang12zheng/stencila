use crate::prelude::*;

use super::quantitative_value::QuantitativeValue;
use super::text::Text;


/// http://schema.org/activityFrequency
/// How often one should engage in the activity.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ActivityFrequencyPropEnum {
    QuantitativeValue(QuantitativeValue),
    Text(Text),
}
