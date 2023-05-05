use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::organization::Organization;
use super::person::Person;

/// The number of interactions for the CreativeWork using the WebSite or SoftwareApplication. The most specific child type of InteractionCounter should be used.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum interactionStatistic {
    CreativeWork(CreativeWork),
    Organization(Organization),
    Person(Person),
}
