use crate::prelude::*;

use super::bus_or_coach::BusOrCoach;
use super::car::Car;

/// The ACRISS Car Classification Code is a code used by many car rental companies, for classifying vehicles. ACRISS stands for Association of Car Rental Industry Systems and Standards.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum acrissCode {
    BusOrCoach(BusOrCoach),
    Car(Car),
}
