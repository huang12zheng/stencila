use crate::prelude::*;

use super::educational_occupational_credential::EducationalOccupationalCredential;
use super::text::Text;


/// http://schema.org/educationRequirements
/// Educational background needed for the position or Occupation.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum EducationRequirementsPropEnum {
    EducationalOccupationalCredential(EducationalOccupationalCredential),
    Text(Text),
}
