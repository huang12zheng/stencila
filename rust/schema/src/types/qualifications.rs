use crate::prelude::*;

use super::job_posting::JobPosting;
use super::occupation::Occupation;

/// Specific qualifications required for this role or Occupation.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum qualifications {
    JobPosting(JobPosting),
    Occupation(Occupation),
}
