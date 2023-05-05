use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/legislationResponsible
/// An individual or organization that has some kind of responsibility for the legislation. Typically the ministry who is/was in charge of elaborating the legislation, or the adressee for potential questions about the legislation once it is published.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum LegislationResponsiblePropEnum {
    Organization(Organization),
    Person(Person),
}
