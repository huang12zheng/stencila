use crate::prelude::*;

use super::educational_occupational_credential::EducationalOccupationalCredential;
use super::text::Text;


/// http://schema.org/qualifications
/// Specific qualifications required for this role or Occupation.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum QualificationsPropEnum {
    EducationalOccupationalCredential(EducationalOccupationalCredential),
    Text(Text),
}
