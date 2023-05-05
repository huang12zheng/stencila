use crate::prelude::*;

use super::date::Date;
use super::date_time::DateTime;


/// http://schema.org/dateCreated
/// The date on which the CreativeWork was created or the item was added to a DataFeed.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum DateCreatedPropEnum {
    Date(Date),
    DateTime(DateTime),
}
