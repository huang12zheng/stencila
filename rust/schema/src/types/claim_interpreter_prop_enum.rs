use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/claimInterpreter
/// For a <a class="localLink" href="/Claim">Claim</a> interpreted from <a class="localLink" href="/MediaObject">MediaObject</a> content     sed to indicate a claim contained, implied or refined from the content of a <a class="localLink" href="/MediaObject">MediaObject</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ClaimInterpreterPropEnum {
    Organization(Organization),
    Person(Person),
}
