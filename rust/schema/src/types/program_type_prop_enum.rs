use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::text::Text;


/// http://schema.org/programType
/// The type of educational or occupational program. For example, classroom, internship, alternance, etc.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ProgramTypePropEnum {
    DefinedTerm(DefinedTerm),
    Text(Text),
}
