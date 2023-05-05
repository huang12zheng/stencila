use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::text::Text;
use super::url::URL;


/// http://schema.org/keywords
/// Keywords or tags used to describe some item. Multiple textual entries in a keywords list are typically delimited by commas, or by repeating the property.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum KeywordsPropEnum {
    DefinedTerm(DefinedTerm),
    Text(Text),
    URL(URL),
}
