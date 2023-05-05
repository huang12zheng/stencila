use crate::prelude::*;

use super::bank_or_credit_union::BankOrCreditUnion;
use super::text::Text;

/// A bank or bank’s branch, financial institution or international financial institution operating the beneficiary’s bank account or releasing funds for the beneficiary.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum beneficiaryBank {
    BankOrCreditUnion(BankOrCreditUnion),
    Text(Text),
}
