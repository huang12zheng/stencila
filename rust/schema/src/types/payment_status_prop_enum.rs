use crate::prelude::*;

use super::payment_status_type::PaymentStatusType;
use super::text::Text;


/// http://schema.org/paymentStatus
/// The status of payment; whether the invoice has been paid or not.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum PaymentStatusPropEnum {
    PaymentStatusType(PaymentStatusType),
    Text(Text),
}
