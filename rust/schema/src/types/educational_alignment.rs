use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::learning_resource::LearningResource;

/// An alignment to an established educational framework.<br/><br/>  This property should not be used where the nature of the alignment can be described using a simple property, for example to express that a resource <a class="localLink" href="/teaches">teaches</a> or <a class="localLink" href="/assesses">assesses</a> a competency.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum educationalAlignment {
    CreativeWork(CreativeWork),
    LearningResource(LearningResource),
}
