use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::text::Text;


/// http://schema.org/teaches
/// The item being described is intended to help a person learn the competency or learning outcome defined by the referenced term.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum TeachesPropEnum {
    DefinedTerm(DefinedTerm),
    Text(Text),
}
