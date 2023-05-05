use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/legislationPassedBy
/// The person or organization that originally passed or made the law: typically parliament (for primary legislation) or government (for secondary legislation). This indicates the "legal author" of the law, as opposed to its physical author.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum LegislationPassedByPropEnum {
    Organization(Organization),
    Person(Person),
}
