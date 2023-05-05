use crate::prelude::*;

use super::event_status_type::EventStatusType;
use super::medical_study_status::MedicalStudyStatus;
use super::text::Text;

/// The status of the study (enumerated).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum status {
    EventStatusType(EventStatusType),
    MedicalStudyStatus(MedicalStudyStatus),
    Text(Text),
}
