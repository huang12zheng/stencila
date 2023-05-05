use crate::prelude::*;

use super::date::Date;
use super::date_time::DateTime;


/// http://schema.org/dateDeleted
/// The datetime the item was removed from the DataFeed.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum DateDeletedPropEnum {
    Date(Date),
    DateTime(DateTime),
}
