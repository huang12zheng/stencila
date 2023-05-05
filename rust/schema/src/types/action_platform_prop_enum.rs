use crate::prelude::*;

use super::digital_platform_enumeration::DigitalPlatformEnumeration;
use super::text::Text;
use super::url::URL;


/// http://schema.org/actionPlatform
/// The high level platform(s) where the Action can be performed for the given URL. To specify a specific application or operating system instance, use actionApplication.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ActionPlatformPropEnum {
    DigitalPlatformEnumeration(DigitalPlatformEnumeration),
    Text(Text),
    URL(URL),
}
