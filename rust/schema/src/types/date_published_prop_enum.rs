use crate::prelude::*;

use super::date::Date;
use super::date_time::DateTime;


/// http://schema.org/datePublished
/// Date of first broadcast/publication.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum DatePublishedPropEnum {
    Date(Date),
    DateTime(DateTime),
}
