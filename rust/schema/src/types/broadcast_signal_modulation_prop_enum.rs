use crate::prelude::*;

use super::qualitative_value::QualitativeValue;
use super::text::Text;


/// http://schema.org/broadcastSignalModulation
/// The modulation (e.g. FM, AM, etc) used by a particular broadcast service.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum BroadcastSignalModulationPropEnum {
    QualitativeValue(QualitativeValue),
    Text(Text),
}
