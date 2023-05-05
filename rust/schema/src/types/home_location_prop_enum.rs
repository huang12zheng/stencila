use crate::prelude::*;

use super::contact_point::ContactPoint;
use super::place::Place;


/// http://schema.org/homeLocation
/// A contact location for a person's residence.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum HomeLocationPropEnum {
    ContactPoint(ContactPoint),
    Place(Place),
}
