use crate::prelude::*;

use super::qualitative_value::QualitativeValue;
use super::text::Text;
use super::url::URL;


/// http://schema.org/engineType
/// The type of engine or engines powering the vehicle.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum EngineTypePropEnum {
    QualitativeValue(QualitativeValue),
    Text(Text),
    URL(URL),
}
