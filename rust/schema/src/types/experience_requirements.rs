use crate::prelude::*;

use super::job_posting::JobPosting;
use super::occupation::Occupation;

/// Description of skills and experience needed for the position or Occupation.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum experienceRequirements {
    JobPosting(JobPosting),
    Occupation(Occupation),
}
