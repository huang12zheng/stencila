use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::text::Text;
use super::url::URL;


/// http://schema.org/credentialCategory
/// The category or type of credential being described, for example "degree”, “certificate”, “badge”, or more specific term.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum CredentialCategoryPropEnum {
    DefinedTerm(DefinedTerm),
    Text(Text),
    URL(URL),
}
