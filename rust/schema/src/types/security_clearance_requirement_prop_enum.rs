use crate::prelude::*;

use super::text::Text;
use super::url::URL;


/// http://schema.org/securityClearanceRequirement
/// A description of any security clearance requirements of the job.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum SecurityClearanceRequirementPropEnum {
    Text(Text),
    URL(URL),
}
