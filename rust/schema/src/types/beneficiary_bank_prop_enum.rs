use crate::prelude::*;

use super::bank_or_credit_union::BankOrCreditUnion;
use super::text::Text;


/// http://schema.org/beneficiaryBank
/// A bank or bank’s branch, financial institution or international financial institution operating the beneficiary’s bank account or releasing funds for the beneficiary.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum BeneficiaryBankPropEnum {
    BankOrCreditUnion(BankOrCreditUnion),
    Text(Text),
}
