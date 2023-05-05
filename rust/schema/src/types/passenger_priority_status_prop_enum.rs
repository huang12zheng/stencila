use crate::prelude::*;

use super::qualitative_value::QualitativeValue;
use super::text::Text;


/// http://schema.org/passengerPriorityStatus
/// The priority status assigned to a passenger for security or boarding (e.g. FastTrack or Priority).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum PassengerPriorityStatusPropEnum {
    QualitativeValue(QualitativeValue),
    Text(Text),
}
