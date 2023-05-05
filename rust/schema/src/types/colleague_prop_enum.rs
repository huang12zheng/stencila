use crate::prelude::*;

use super::person::Person;
use super::url::URL;


/// http://schema.org/colleague
/// A colleague of the person.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ColleaguePropEnum {
    Person(Person),
    URL(URL),
}
