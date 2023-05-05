use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::education_event::EducationEvent;
use super::educational_occupational_credential::EducationalOccupationalCredential;
use super::learning_resource::LearningResource;

/// The level in terms of progression through an educational or training context. Examples of educational levels include 'beginner', 'intermediate' or 'advanced', and formal sets of level indicators.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum educationalLevel {
    CreativeWork(CreativeWork),
    EducationEvent(EducationEvent),
    EducationalOccupationalCredential(EducationalOccupationalCredential),
    LearningResource(LearningResource),
}
