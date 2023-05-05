use crate::prelude::*;

use super::gender_type::GenderType;
use super::text::Text;


/// http://schema.org/gender
/// Gender of something, typically a <a class="localLink" href="/Person">Person</a>, but possibly also fictional characters, animals, etc. While http://schema.org/Male and http://schema.org/Female may be used, text strings are also acceptable for people who do not identify as a binary gender. The <a class="localLink" href="/gender">gender</a> property can also be used in an extended sense to cover e.g. the gender of sports teams. As with the gender of individuals, we do not try to enumerate all possibilities. A mixed-gender <a class="localLink" href="/SportsTeam">SportsTeam</a> can be indicated with a text value of "Mixed".
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum GenderPropEnum {
    GenderType(GenderType),
    Text(Text),
}
