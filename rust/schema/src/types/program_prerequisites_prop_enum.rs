use crate::prelude::*;

use super::alignment_object::AlignmentObject;
use super::course::Course;
use super::educational_occupational_credential::EducationalOccupationalCredential;
use super::text::Text;


/// http://schema.org/programPrerequisites
/// Prerequisites for enrolling in the program.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ProgramPrerequisitesPropEnum {
    AlignmentObject(AlignmentObject),
    Course(Course),
    EducationalOccupationalCredential(EducationalOccupationalCredential),
    Text(Text),
}
