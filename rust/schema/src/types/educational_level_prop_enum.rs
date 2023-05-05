use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::text::Text;
use super::url::URL;


/// http://schema.org/educationalLevel
/// The level in terms of progression through an educational or training context. Examples of educational levels include 'beginner', 'intermediate' or 'advanced', and formal sets of level indicators.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum EducationalLevelPropEnum {
    DefinedTerm(DefinedTerm),
    Text(Text),
    URL(URL),
}
