use crate::prelude::*;

use super::car_usage_type::CarUsageType;
use super::text::Text;

/// Indicates whether the vehicle has been used for special purposes, like commercial rental, driving school, or as a taxi. The legislation in many countries requires this information to be revealed when offering a car for sale.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum vehicleSpecialUsage {
    CarUsageType(CarUsageType),
    Text(Text),
}
