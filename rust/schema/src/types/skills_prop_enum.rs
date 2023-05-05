use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::text::Text;


/// http://schema.org/skills
/// A statement of knowledge, skill, ability, task or any other assertion expressing a competency that is desired or required to fulfill this role or to work in this occupation.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum SkillsPropEnum {
    DefinedTerm(DefinedTerm),
    Text(Text),
}
