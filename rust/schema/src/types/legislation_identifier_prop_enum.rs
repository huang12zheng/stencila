use crate::prelude::*;

use super::text::Text;
use super::url::URL;


/// http://schema.org/legislationIdentifier
/// An identifier for the legislation. This can be either a string-based identifier, like the CELEX at EU level or the NOR in France, or a web-based, URL/URI identifier, like an ELI (European Legislation Identifier) or an URN-Lex.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum LegislationIdentifierPropEnum {
    Text(Text),
    URL(URL),
}
