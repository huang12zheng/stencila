use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;

/// For a <a class="localLink" href="/Claim">Claim</a> interpreted from <a class="localLink" href="/MediaObject">MediaObject</a> content     sed to indicate a claim contained, implied or refined from the content of a <a class="localLink" href="/MediaObject">MediaObject</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum claimInterpreter {
    Organization(Organization),
    Person(Person),
}
