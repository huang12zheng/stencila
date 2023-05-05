use crate::prelude::*;

use super::qualitative_value::QualitativeValue;
use super::text::Text;
use super::url::URL;


/// http://schema.org/fuelType
/// The type of fuel suitable for the engine or engines of the vehicle. If the vehicle has only one engine, this property can be attached directly to the vehicle.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum FuelTypePropEnum {
    QualitativeValue(QualitativeValue),
    Text(Text),
    URL(URL),
}
