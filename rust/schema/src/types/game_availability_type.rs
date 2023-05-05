use crate::prelude::*;

use super::game_availability_enumeration::GameAvailabilityEnumeration;
use super::text::Text;

/// Indicates the availability type of the game content associated with this action, such as whether it is a full version or a demo.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum gameAvailabilityType {
    GameAvailabilityEnumeration(GameAvailabilityEnumeration),
    Text(Text),
}
