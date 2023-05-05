use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// An individual or organization that has some kind of responsibility for the legislation. Typically the ministry who is/was in charge of elaborating the legislation, or the adressee for potential questions about the legislation once it is published.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum legislationResponsible {
    Organization(Organization),
    Person(Person),
}
