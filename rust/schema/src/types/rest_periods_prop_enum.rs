use crate::prelude::*;

use super::quantitative_value::QuantitativeValue;
use super::text::Text;


/// http://schema.org/restPeriods
/// How often one should break from the activity.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum RestPeriodsPropEnum {
    QuantitativeValue(QuantitativeValue),
    Text(Text),
}
