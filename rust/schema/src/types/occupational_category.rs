use crate::prelude::*;

use super::educational_occupational_program::EducationalOccupationalProgram;
use super::job_posting::JobPosting;
use super::occupation::Occupation;
use super::work_based_program::WorkBasedProgram;

/// A category describing the job, preferably using a term from a taxonomy such as <a href="http://www.onetcenter.org/taxonomy.html">BLS O*NET-SOC</a>, <a href="https://www.ilo.org/public/english/bureau/stat/isco/isco08/">ISCO-08</a> or similar, with the property repeated for each applicable value. Ideally the taxonomy should be identified, and both the textual label and formal code for the category should be provided.<br/><br/>  Note: for historical reasons, any textual label and formal code provided as a literal may be assumed to be from O*NET-SOC.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum occupationalCategory {
    EducationalOccupationalProgram(EducationalOccupationalProgram),
    JobPosting(JobPosting),
    Occupation(Occupation),
    WorkBasedProgram(WorkBasedProgram),
}
