use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::event::Event;


/// http://schema.org/subjectOf
/// A CreativeWork or Event about this Thing.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum SubjectOfPropEnum {
    CreativeWork(CreativeWork),
    Event(Event),
}
