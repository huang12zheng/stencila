use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::text::Text;
use super::url::URL;

/// The category or type of credential being described, for example "degree”, “certificate”, “badge”, or more specific term.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum credentialCategory {
    DefinedTerm(DefinedTerm),
    Text(Text),
    URL(URL),
}
