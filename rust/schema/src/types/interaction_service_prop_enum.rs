use crate::prelude::*;

use super::software_application::SoftwareApplication;
use super::web_site::WebSite;


/// http://schema.org/interactionService
/// The WebSite or SoftwareApplication where the interactions took place.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum InteractionServicePropEnum {
    SoftwareApplication(SoftwareApplication),
    WebSite(WebSite),
}
