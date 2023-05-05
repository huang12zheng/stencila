use crate::prelude::*;

use super::broadcast_frequency_specification::BroadcastFrequencySpecification;
use super::text::Text;


/// http://schema.org/broadcastFrequency
/// The frequency used for over-the-air broadcasts. Numeric values or simple ranges, e.g. 87-99. In addition a shortcut idiom is supported for frequences of AM and FM radio channels, e.g. "87 FM".
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum BroadcastFrequencyPropEnum {
    BroadcastFrequencySpecification(BroadcastFrequencySpecification),
    Text(Text),
}
