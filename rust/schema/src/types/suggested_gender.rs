use crate::prelude::*;

use super::gender_type::GenderType;
use super::text::Text;

/// The suggested gender of the intended person or audience, for example "male", "female", or "unisex".
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum suggestedGender {
    GenderType(GenderType),
    Text(Text),
}
