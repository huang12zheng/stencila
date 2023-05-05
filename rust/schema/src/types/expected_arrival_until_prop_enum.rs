use crate::prelude::*;

use super::date::Date;
use super::date_time::DateTime;


/// http://schema.org/expectedArrivalUntil
/// The latest date the package may arrive.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ExpectedArrivalUntilPropEnum {
    Date(Date),
    DateTime(DateTime),
}
