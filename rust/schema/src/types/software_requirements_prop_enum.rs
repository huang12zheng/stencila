use crate::prelude::*;

use super::text::Text;
use super::url::URL;


/// http://schema.org/softwareRequirements
/// Component dependency requirements for application. This includes runtime environments and shared libraries that are not included in the application distribution package, but required to run the application (examples: DirectX, Java or .NET runtime).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum SoftwareRequirementsPropEnum {
    Text(Text),
    URL(URL),
}
