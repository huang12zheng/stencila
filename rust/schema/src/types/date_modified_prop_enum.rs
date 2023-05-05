use crate::prelude::*;

use super::date::Date;
use super::date_time::DateTime;


/// http://schema.org/dateModified
/// The date on which the CreativeWork was most recently modified or when the item's entry was modified within a DataFeed.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum DateModifiedPropEnum {
    Date(Date),
    DateTime(DateTime),
}
