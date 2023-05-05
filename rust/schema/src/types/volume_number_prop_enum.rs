use crate::prelude::*;

use super::integer::Integer;
use super::text::Text;


/// http://schema.org/volumeNumber
/// Identifies the volume of publication or multi-part work; for example, "iii" or "2".
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum VolumeNumberPropEnum {
    Integer(Integer),
    Text(Text),
}
