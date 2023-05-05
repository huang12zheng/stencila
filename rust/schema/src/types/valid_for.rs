use crate::prelude::*;

use super::educational_occupational_credential::EducationalOccupationalCredential;
use super::permit::Permit;

/// The duration of validity of a permit or similar thing.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum validFor {
    EducationalOccupationalCredential(EducationalOccupationalCredential),
    Permit(Permit),
}
