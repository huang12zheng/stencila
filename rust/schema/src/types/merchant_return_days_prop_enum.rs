use crate::prelude::*;

use super::date::Date;
use super::date_time::DateTime;
use super::integer::Integer;


/// http://schema.org/merchantReturnDays
/// Specifies either a fixed return date or the number of days (from the delivery date) that a product can be returned. Used when the <a class="localLink" href="/returnPolicyCategory">returnPolicyCategory</a> property is specified as <a class="localLink" href="/MerchantReturnFiniteReturnWindow">MerchantReturnFiniteReturnWindow</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum MerchantReturnDaysPropEnum {
    Date(Date),
    DateTime(DateTime),
    Integer(Integer),
}
