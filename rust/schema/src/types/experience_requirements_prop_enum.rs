use crate::prelude::*;

use super::occupational_experience_requirements::OccupationalExperienceRequirements;
use super::text::Text;


/// http://schema.org/experienceRequirements
/// Description of skills and experience needed for the position or Occupation.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ExperienceRequirementsPropEnum {
    OccupationalExperienceRequirements(OccupationalExperienceRequirements),
    Text(Text),
}
