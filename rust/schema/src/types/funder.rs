use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::event::Event;
use super::grant::Grant;
use super::monetary_grant::MonetaryGrant;
use super::organization::Organization;
use super::person::Person;

/// A person or organization that supports (sponsors) something through some kind of financial contribution.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum funder {
    CreativeWork(CreativeWork),
    Event(Event),
    Grant(Grant),
    MonetaryGrant(MonetaryGrant),
    Organization(Organization),
    Person(Person),
}
