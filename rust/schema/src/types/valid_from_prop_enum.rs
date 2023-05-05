use crate::prelude::*;

use super::date::Date;
use super::date_time::DateTime;


/// http://schema.org/validFrom
/// The date when the item becomes valid.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ValidFromPropEnum {
    Date(Date),
    DateTime(DateTime),
}
