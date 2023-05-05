use crate::prelude::*;

use super::employee_role::EmployeeRole;
use super::job_posting::JobPosting;

/// The base salary of the job or of an employee in an EmployeeRole.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum baseSalary {
    EmployeeRole(EmployeeRole),
    JobPosting(JobPosting),
}
