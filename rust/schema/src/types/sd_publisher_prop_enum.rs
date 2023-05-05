use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/sdPublisher
/// Indicates the party responsible for generating and publishing the current structured data markup, typically in cases where the structured data is derived automatically from existing published content but published on a different site. For example, student projects and open data initiatives often re-publish existing content with more explicitly structured metadata. The <a class="localLink" href="/sdPublisher">sdPublisher</a> property helps make such practices more explicit.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum SdPublisherPropEnum {
    Organization(Organization),
    Person(Person),
}
