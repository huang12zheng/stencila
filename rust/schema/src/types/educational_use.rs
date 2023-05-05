use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::learning_resource::LearningResource;

/// The purpose of a work in the context of education; for example, 'assignment', 'group work'.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum educationalUse {
    CreativeWork(CreativeWork),
    LearningResource(LearningResource),
}
