use crate::prelude::*;

use super::quantitative_value::QuantitativeValue;
use super::text::Text;

/// How often one should engage in the activity.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum activityFrequency {
    QuantitativeValue(QuantitativeValue),
    Text(Text),
}
