use crate::prelude::*;

use super::educational_occupational_credential::EducationalOccupationalCredential;
use super::text::Text;
use super::url::URL;

/// A description of the qualification, award, certificate, diploma or other educational credential awarded as a consequence of successful completion of this course or program.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum educationalCredentialAwarded {
    EducationalOccupationalCredential(EducationalOccupationalCredential),
    Text(Text),
    URL(URL),
}
