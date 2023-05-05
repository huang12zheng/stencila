use crate::prelude::*;

use super::loan_or_credit::LoanOrCredit;
use super::payment_method::PaymentMethod;

/// The payment method(s) accepted by seller for this offer.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum acceptedPaymentMethod {
    LoanOrCredit(LoanOrCredit),
    PaymentMethod(PaymentMethod),
}
