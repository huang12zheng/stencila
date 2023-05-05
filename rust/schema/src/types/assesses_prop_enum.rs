use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::text::Text;


/// http://schema.org/assesses
/// The item being described is intended to assess the competency or learning outcome defined by the referenced term.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum AssessesPropEnum {
    DefinedTerm(DefinedTerm),
    Text(Text),
}
