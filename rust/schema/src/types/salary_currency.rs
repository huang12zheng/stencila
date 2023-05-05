use crate::prelude::*;

use super::employee_role::EmployeeRole;
use super::job_posting::JobPosting;

/// The currency (coded using <a href="http://en.wikipedia.org/wiki/ISO_4217">ISO 4217</a>) used for the main salary information in this job posting or for this employee.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum salaryCurrency {
    EmployeeRole(EmployeeRole),
    JobPosting(JobPosting),
}
