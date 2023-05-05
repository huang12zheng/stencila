use crate::prelude::*;

use super::educational_occupational_credential::EducationalOccupationalCredential;
use super::permit::Permit;

/// The geographic area where a permit or similar thing is valid.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum validIn {
    EducationalOccupationalCredential(EducationalOccupationalCredential),
    Permit(Permit),
}
