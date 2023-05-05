use crate::prelude::*;

use super::text::Text;
use super::url::URL;


/// http://schema.org/bankAccountType
/// The type of a bank account.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum BankAccountTypePropEnum {
    Text(Text),
    URL(URL),
}
