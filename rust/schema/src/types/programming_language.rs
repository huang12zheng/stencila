use crate::prelude::*;

use super::computer_language::ComputerLanguage;
use super::text::Text;

/// The computer programming language.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum programmingLanguage {
    ComputerLanguage(ComputerLanguage),
    Text(Text),
}
