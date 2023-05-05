use crate::prelude::*;

use super::event_status_type::EventStatusType;
use super::medical_study_status::MedicalStudyStatus;
use super::text::Text;


/// http://schema.org/status
/// The status of the study (enumerated).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum StatusPropEnum {
    EventStatusType(EventStatusType),
    MedicalStudyStatus(MedicalStudyStatus),
    Text(Text),
}
