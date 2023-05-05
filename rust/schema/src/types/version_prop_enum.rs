use crate::prelude::*;

use super::number::Number;
use super::text::Text;


/// http://schema.org/version
/// The version of the CreativeWork embodied by a specified resource.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum VersionPropEnum {
    Number(Number),
    Text(Text),
}
