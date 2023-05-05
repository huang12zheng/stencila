use crate::prelude::*;

use super::date_time::DateTime;
use super::text::Text;


/// http://schema.org/cvdCollectionDate
/// collectiondate - Date for which patient counts are reported.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum CvdCollectionDatePropEnum {
    DateTime(DateTime),
    Text(Text),
}
