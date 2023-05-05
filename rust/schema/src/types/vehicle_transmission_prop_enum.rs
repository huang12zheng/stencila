use crate::prelude::*;

use super::qualitative_value::QualitativeValue;
use super::text::Text;
use super::url::URL;


/// http://schema.org/vehicleTransmission
/// The type of component used for transmitting the power from a rotating power source to the wheels or other relevant component(s) ("gearbox" for cars).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum VehicleTransmissionPropEnum {
    QualitativeValue(QualitativeValue),
    Text(Text),
    URL(URL),
}
