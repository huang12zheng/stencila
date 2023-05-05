use crate::prelude::*;

use super::duration::Duration;
use super::quantitative_value::QuantitativeValue;


/// http://schema.org/activityDuration
/// Length of time to engage in the activity.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ActivityDurationPropEnum {
    Duration(Duration),
    QuantitativeValue(QuantitativeValue),
}
