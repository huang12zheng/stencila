use crate::prelude::*;

use super::business_audience::BusinessAudience;
use super::organization::Organization;

/// The number of employees in an organization, e.g. business.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum numberOfEmployees {
    BusinessAudience(BusinessAudience),
    Organization(Organization),
}
