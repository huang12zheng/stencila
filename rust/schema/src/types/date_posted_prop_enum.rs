use crate::prelude::*;

use super::date::Date;
use super::date_time::DateTime;


/// http://schema.org/datePosted
/// Publication date of an online listing.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum DatePostedPropEnum {
    Date(Date),
    DateTime(DateTime),
}
