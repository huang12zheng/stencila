use crate::prelude::*;

use super::alignment_object::AlignmentObject;
use super::course::Course;
use super::educational_occupational_credential::EducationalOccupationalCredential;
use super::text::Text;

/// Prerequisites for enrolling in the program.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum programPrerequisites {
    AlignmentObject(AlignmentObject),
    Course(Course),
    EducationalOccupationalCredential(EducationalOccupationalCredential),
    Text(Text),
}
