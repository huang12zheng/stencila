use crate::prelude::*;

use super::date::Date;
use super::date_time::DateTime;


/// http://schema.org/validThrough
/// The date after when the item is not valid. For example the end of an offer, salary period, or a period of opening hours.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ValidThroughPropEnum {
    Date(Date),
    DateTime(DateTime),
}
