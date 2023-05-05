use crate::prelude::*;

use super::brand::Brand;
use super::organization::Organization;

/// The brand(s) associated with a product or service, or the brand(s) maintained by an organization or business person.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum brand {
    Brand(Brand),
    Organization(Organization),
}
