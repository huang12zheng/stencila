use crate::prelude::*;

use super::educational_occupational_credential::EducationalOccupationalCredential;
use super::text::Text;

/// Specific qualifications required for this role or Occupation.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum qualifications {
    EducationalOccupationalCredential(EducationalOccupationalCredential),
    Text(Text),
}
