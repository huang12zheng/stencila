use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/customer
/// Party placing the order or paying the invoice.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum CustomerPropEnum {
    Organization(Organization),
    Person(Person),
}
