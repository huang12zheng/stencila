use crate::prelude::*;

use super::job_posting::JobPosting;
use super::occupation::Occupation;

/// An estimated salary for a job posting or occupation, based on a variety of variables including, but not limited to industry, job title, and location. Estimated salaries  are often computed by outside organizations rather than the hiring organization, who may not have committed to the estimated value.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum estimatedSalary {
    JobPosting(JobPosting),
    Occupation(Occupation),
}
