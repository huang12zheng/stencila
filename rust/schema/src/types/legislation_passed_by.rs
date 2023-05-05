use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// The person or organization that originally passed or made the law: typically parliament (for primary legislation) or government (for secondary legislation). This indicates the "legal author" of the law, as opposed to its physical author.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum legislationPassedBy {
    Organization(Organization),
    Person(Person),
}
