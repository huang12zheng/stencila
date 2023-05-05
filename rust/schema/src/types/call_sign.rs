use crate::prelude::*;

use super::broadcast_service::BroadcastService;
use super::person::Person;
use super::vehicle::Vehicle;

/// A <a href="https://en.wikipedia.org/wiki/Call_sign">callsign</a>, as used in broadcasting and radio communications to identify people, radio and TV stations, or vehicles.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum callSign {
    BroadcastService(BroadcastService),
    Person(Person),
    Vehicle(Vehicle),
}
