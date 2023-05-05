use crate::prelude::*;

use super::occupational_experience_requirements::OccupationalExperienceRequirements;
use super::text::Text;

/// Description of skills and experience needed for the position or Occupation.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum experienceRequirements {
    OccupationalExperienceRequirements(OccupationalExperienceRequirements),
    Text(Text),
}
