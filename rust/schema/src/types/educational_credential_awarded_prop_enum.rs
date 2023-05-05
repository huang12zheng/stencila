use crate::prelude::*;

use super::educational_occupational_credential::EducationalOccupationalCredential;
use super::text::Text;
use super::url::URL;


/// http://schema.org/educationalCredentialAwarded
/// A description of the qualification, award, certificate, diploma or other educational credential awarded as a consequence of successful completion of this course or program.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum EducationalCredentialAwardedPropEnum {
    EducationalOccupationalCredential(EducationalOccupationalCredential),
    Text(Text),
    URL(URL),
}
