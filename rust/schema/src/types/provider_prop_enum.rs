use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/provider
/// The service provider, service operator, or service performer; the goods producer. Another party (a seller) may offer those services or goods on behalf of the provider. A provider may also serve as the seller.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ProviderPropEnum {
    Organization(Organization),
    Person(Person),
}
