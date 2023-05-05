use crate::prelude::*;

use super::computer_language::ComputerLanguage;
use super::text::Text;


/// http://schema.org/programmingLanguage
/// The computer programming language.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ProgrammingLanguagePropEnum {
    ComputerLanguage(ComputerLanguage),
    Text(Text),
}
