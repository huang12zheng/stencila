use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/broker
/// An entity that arranges for an exchange between a buyer and a seller.  In most cases a broker never acquires or releases ownership of a product or service involved in an exchange.  If it is not clear whether an entity is a broker, seller, or buyer, the latter two terms are preferred.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum BrokerPropEnum {
    Organization(Organization),
    Person(Person),
}
