use crate::prelude::*;

use super::educational_occupational_program::EducationalOccupationalProgram;
use super::work_based_program::WorkBasedProgram;

/// The estimated salary earned while in the program.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum trainingSalary {
    EducationalOccupationalProgram(EducationalOccupationalProgram),
    WorkBasedProgram(WorkBasedProgram),
}
