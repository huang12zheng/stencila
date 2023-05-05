use crate::prelude::*;

use super::loan_or_credit::LoanOrCredit;
use super::payment_method::PaymentMethod;


/// http://schema.org/acceptedPaymentMethod
/// The payment method(s) accepted by seller for this offer.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum AcceptedPaymentMethodPropEnum {
    LoanOrCredit(LoanOrCredit),
    PaymentMethod(PaymentMethod),
}
