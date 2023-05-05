use crate::prelude::*;

use super::qualitative_value::QualitativeValue;
use super::text::Text;
use super::url::URL;


/// http://schema.org/meetsEmissionStandard
/// Indicates that the vehicle meets the respective emission standard.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum MeetsEmissionStandardPropEnum {
    QualitativeValue(QualitativeValue),
    Text(Text),
    URL(URL),
}
