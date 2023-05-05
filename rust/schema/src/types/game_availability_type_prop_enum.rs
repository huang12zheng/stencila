use crate::prelude::*;

use super::game_availability_enumeration::GameAvailabilityEnumeration;
use super::text::Text;


/// http://schema.org/gameAvailabilityType
/// Indicates the availability type of the game content associated with this action, such as whether it is a full version or a demo.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum GameAvailabilityTypePropEnum {
    GameAvailabilityEnumeration(GameAvailabilityEnumeration),
    Text(Text),
}
