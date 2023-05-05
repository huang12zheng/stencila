use crate::prelude::*;

use super::educational_occupational_credential::EducationalOccupationalCredential;
use super::learning_resource::LearningResource;

/// Knowledge, skill, ability or personal attribute that must be demonstrated by a person or other entity in order to do something such as earn an Educational Occupational Credential or understand a LearningResource.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum competencyRequired {
    EducationalOccupationalCredential(EducationalOccupationalCredential),
    LearningResource(LearningResource),
}
