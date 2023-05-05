use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::text::Text;


/// http://schema.org/financialAidEligible
/// A financial aid type or program which students may use to pay for tuition or fees associated with the program.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum FinancialAidEligiblePropEnum {
    DefinedTerm(DefinedTerm),
    Text(Text),
}
