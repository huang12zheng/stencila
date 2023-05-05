use crate::prelude::*;

use super::duration::Duration;
use super::quantitative_value::QuantitativeValue;

/// Length of time to engage in the activity.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum activityDuration {
    Duration(Duration),
    QuantitativeValue(QuantitativeValue),
}
