use crate::prelude::*;

use super::medical_audience::MedicalAudience;
use super::medical_audience_type::MedicalAudienceType;

/// Medical audience for page.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum medicalAudience {
    MedicalAudience(MedicalAudience),
    MedicalAudienceType(MedicalAudienceType),
}
