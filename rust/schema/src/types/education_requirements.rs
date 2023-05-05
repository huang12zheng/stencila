use crate::prelude::*;

use super::job_posting::JobPosting;
use super::occupation::Occupation;

/// Educational background needed for the position or Occupation.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum educationRequirements {
    JobPosting(JobPosting),
    Occupation(Occupation),
}
