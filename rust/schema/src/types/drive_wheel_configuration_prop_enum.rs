use crate::prelude::*;

use super::drive_wheel_configuration_value::DriveWheelConfigurationValue;
use super::text::Text;


/// http://schema.org/driveWheelConfiguration
/// The drive wheel configuration, i.e. which roadwheels will receive torque from the vehicle's engine via the drivetrain.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum DriveWheelConfigurationPropEnum {
    DriveWheelConfigurationValue(DriveWheelConfigurationValue),
    Text(Text),
}
