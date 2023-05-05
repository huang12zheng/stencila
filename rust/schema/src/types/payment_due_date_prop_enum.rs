use crate::prelude::*;

use super::date::Date;
use super::date_time::DateTime;


/// http://schema.org/paymentDueDate
/// The date that payment is due.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum PaymentDueDatePropEnum {
    Date(Date),
    DateTime(DateTime),
}
