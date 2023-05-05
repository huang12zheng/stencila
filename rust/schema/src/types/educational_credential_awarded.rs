use crate::prelude::*;

use super::course::Course;
use super::educational_occupational_program::EducationalOccupationalProgram;

/// A description of the qualification, award, certificate, diploma or other educational credential awarded as a consequence of successful completion of this course or program.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum educationalCredentialAwarded {
    Course(Course),
    EducationalOccupationalProgram(EducationalOccupationalProgram),
}
