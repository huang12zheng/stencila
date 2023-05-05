use crate::prelude::*;

use super::bio_chem_entity::BioChemEntity;
use super::creative_work::CreativeWork;
use super::event::Event;
use super::medical_entity::MedicalEntity;
use super::organization::Organization;
use super::person::Person;
use super::product::Product;

/// Indicates something directly or indirectly funded or sponsored through a <a class="localLink" href="/Grant">Grant</a>. See also <a class="localLink" href="/ownershipFundingInfo">ownershipFundingInfo</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum fundedItem {
    BioChemEntity(BioChemEntity),
    CreativeWork(CreativeWork),
    Event(Event),
    MedicalEntity(MedicalEntity),
    Organization(Organization),
    Person(Person),
    Product(Product),
}
