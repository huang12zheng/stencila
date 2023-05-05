use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::text::Text;


/// http://schema.org/educationalUse
/// The purpose of a work in the context of education; for example, 'assignment', 'group work'.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum EducationalUsePropEnum {
    DefinedTerm(DefinedTerm),
    Text(Text),
}
