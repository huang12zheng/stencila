use crate::prelude::*;

use super::date::Date;
use super::text::Text;


/// http://schema.org/jobStartDate
/// The date on which a successful applicant for this job would be expected to start work. Choose a specific date in the future or use the jobImmediateStart property to indicate the position is to be filled as soon as possible.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum JobStartDatePropEnum {
    Date(Date),
    Text(Text),
}
