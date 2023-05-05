use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::learning_resource::LearningResource;

/// The predominant type or kind characterizing the learning resource. For example, 'presentation', 'handout'.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum learningResourceType {
    CreativeWork(CreativeWork),
    LearningResource(LearningResource),
}
