use crate::prelude::*;

use super::course::Course;
use super::educational_occupational_program::EducationalOccupationalProgram;

/// The number of credits or units awarded by a Course or required to complete an EducationalOccupationalProgram.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum numberOfCredits {
    Course(Course),
    EducationalOccupationalProgram(EducationalOccupationalProgram),
}
