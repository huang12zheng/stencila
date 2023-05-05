use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// An entity which offers (sells / leases / lends / loans) the services / goods.  A seller may also be a provider.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum seller {
    Organization(Organization),
    Person(Person),
}
