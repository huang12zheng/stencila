use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::education_event::EducationEvent;
use super::learning_resource::LearningResource;

/// The item being described is intended to assess the competency or learning outcome defined by the referenced term.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum assesses {
    CreativeWork(CreativeWork),
    EducationEvent(EducationEvent),
    LearningResource(LearningResource),
}
