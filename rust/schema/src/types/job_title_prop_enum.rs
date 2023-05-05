use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::text::Text;


/// http://schema.org/jobTitle
/// The job title of the person (for example, Financial Manager).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum JobTitlePropEnum {
    DefinedTerm(DefinedTerm),
    Text(Text),
}
