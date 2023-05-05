use crate::prelude::*;

use super::duration::Duration;
use super::text::Text;


/// http://schema.org/estimatedFlightDuration
/// The estimated time the flight will take.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum EstimatedFlightDurationPropEnum {
    Duration(Duration),
    Text(Text),
}
