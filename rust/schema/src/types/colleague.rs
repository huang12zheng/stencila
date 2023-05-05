use crate::prelude::*;

use super::person::Person;
use super::url::URL;

/// A colleague of the person.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum colleague {
    Person(Person),
    URL(URL),
}
