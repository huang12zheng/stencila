use crate::prelude::*;

use super::gender_type::GenderType;
use super::text::Text;


/// http://schema.org/suggestedGender
/// The suggested gender of the intended person or audience, for example "male", "female", or "unisex".
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum SuggestedGenderPropEnum {
    GenderType(GenderType),
    Text(Text),
}
