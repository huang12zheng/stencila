use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::text::Text;

/// The predominant type or kind characterizing the learning resource. For example, 'presentation', 'handout'.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum learningResourceType {
    DefinedTerm(DefinedTerm),
    Text(Text),
}
