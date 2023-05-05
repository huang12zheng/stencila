use crate::prelude::*;

use super::contact_point::ContactPoint;
use super::place::Place;


/// http://schema.org/workLocation
/// A contact location for a person's place of work.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum WorkLocationPropEnum {
    ContactPoint(ContactPoint),
    Place(Place),
}
