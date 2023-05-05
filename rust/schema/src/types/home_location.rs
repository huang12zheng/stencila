use crate::prelude::*;

use super::contact_point::ContactPoint;
use super::place::Place;

/// A contact location for a person's residence.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum homeLocation {
    ContactPoint(ContactPoint),
    Place(Place),
}
