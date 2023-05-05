use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::text::Text;


/// http://schema.org/learningResourceType
/// The predominant type or kind characterizing the learning resource. For example, 'presentation', 'handout'.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum LearningResourceTypePropEnum {
    DefinedTerm(DefinedTerm),
    Text(Text),
}
