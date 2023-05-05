use crate::prelude::*;

use super::drive_wheel_configuration_value::DriveWheelConfigurationValue;
use super::text::Text;

/// The drive wheel configuration, i.e. which roadwheels will receive torque from the vehicle's engine via the drivetrain.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum driveWheelConfiguration {
    DriveWheelConfigurationValue(DriveWheelConfigurationValue),
    Text(Text),
}
