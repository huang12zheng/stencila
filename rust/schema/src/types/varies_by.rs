use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::text::Text;

/// Indicates the property or properties by which the variants in a <a class="localLink" href="/ProductGroup">ProductGroup</a> vary, e.g. their size, color etc. Schema.org properties can be referenced by their short name e.g. "color"; terms defined elsewhere can be referenced with their URIs.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum variesBy {
    DefinedTerm(DefinedTerm),
    Text(Text),
}
