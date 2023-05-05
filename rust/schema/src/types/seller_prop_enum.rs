use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/seller
/// An entity which offers (sells / leases / lends / loans) the services / goods.  A seller may also be a provider.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum SellerPropEnum {
    Organization(Organization),
    Person(Person),
}
