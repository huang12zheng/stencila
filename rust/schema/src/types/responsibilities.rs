use crate::prelude::*;

use super::job_posting::JobPosting;
use super::occupation::Occupation;

/// Responsibilities associated with this role or Occupation.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum responsibilities {
    JobPosting(JobPosting),
    Occupation(Occupation),
}
