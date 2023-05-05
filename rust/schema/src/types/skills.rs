use crate::prelude::*;

use super::job_posting::JobPosting;
use super::occupation::Occupation;

/// A statement of knowledge, skill, ability, task or any other assertion expressing a competency that is desired or required to fulfill this role or to work in this occupation.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum skills {
    JobPosting(JobPosting),
    Occupation(Occupation),
}
