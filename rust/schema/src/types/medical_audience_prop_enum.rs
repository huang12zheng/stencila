use crate::prelude::*;

use super::medical_audience::MedicalAudience;
use super::medical_audience_type::MedicalAudienceType;


/// http://schema.org/medicalAudience
/// Medical audience for page.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum MedicalAudiencePropEnum {
    MedicalAudience(MedicalAudience),
    MedicalAudienceType(MedicalAudienceType),
}
