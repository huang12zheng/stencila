use crate::prelude::*;

use super::text::Text;
use super::url::URL;

/// A description of any security clearance requirements of the job.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum securityClearanceRequirement {
    Text(Text),
    URL(URL),
}
