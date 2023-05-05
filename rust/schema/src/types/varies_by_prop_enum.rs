use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::text::Text;


/// http://schema.org/variesBy
/// Indicates the property or properties by which the variants in a <a class="localLink" href="/ProductGroup">ProductGroup</a> vary, e.g. their size, color etc. Schema.org properties can be referenced by their short name e.g. "color"; terms defined elsewhere can be referenced with their URIs.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum VariesByPropEnum {
    DefinedTerm(DefinedTerm),
    Text(Text),
}
