use crate::prelude::*;

use super::text::Text;
use super::url::URL;


/// http://schema.org/loanType
/// The type of a loan or credit.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum LoanTypePropEnum {
    Text(Text),
    URL(URL),
}
