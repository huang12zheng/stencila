use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::text::Text;

/// A financial aid type or program which students may use to pay for tuition or fees associated with the program.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum financialAidEligible {
    DefinedTerm(DefinedTerm),
    Text(Text),
}
