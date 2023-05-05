use crate::prelude::*;

use super::payment_status_type::PaymentStatusType;
use super::text::Text;

/// The status of payment; whether the invoice has been paid or not.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum paymentStatus {
    PaymentStatusType(PaymentStatusType),
    Text(Text),
}
