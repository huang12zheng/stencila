use crate::prelude::*;

use super::defined_term_set::DefinedTermSet;
use super::url::URL;


/// http://schema.org/inDefinedTermSet
/// A <a class="localLink" href="/DefinedTermSet">DefinedTermSet</a> that contains this term.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum InDefinedTermSetPropEnum {
    DefinedTermSet(DefinedTermSet),
    URL(URL),
}
