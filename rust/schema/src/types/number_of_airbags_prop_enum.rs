use crate::prelude::*;

use super::number::Number;
use super::text::Text;


/// http://schema.org/numberOfAirbags
/// The number or type of airbags in the vehicle.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum NumberOfAirbagsPropEnum {
    Number(Number),
    Text(Text),
}
