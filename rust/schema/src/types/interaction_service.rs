use crate::prelude::*;

use super::software_application::SoftwareApplication;
use super::web_site::WebSite;

/// The WebSite or SoftwareApplication where the interactions took place.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum interactionService {
    SoftwareApplication(SoftwareApplication),
    WebSite(WebSite),
}
