use crate::prelude::*;

use super::date::Date;
use super::date_time::DateTime;


/// http://schema.org/orderDate
/// Date order was placed.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum OrderDatePropEnum {
    Date(Date),
    DateTime(DateTime),
}