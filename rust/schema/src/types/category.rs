use crate::prelude::*;

use super::action_access_specification::ActionAccessSpecification;
use super::invoice::Invoice;
use super::offer::Offer;
use super::physical_activity::PhysicalActivity;
use super::product::Product;
use super::recommendation::Recommendation;
use super::service::Service;
use super::special_announcement::SpecialAnnouncement;

/// A category for the item. Greater signs or slashes can be used to informally indicate a category hierarchy.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum category {
    ActionAccessSpecification(ActionAccessSpecification),
    Invoice(Invoice),
    Offer(Offer),
    PhysicalActivity(PhysicalActivity),
    Product(Product),
    Recommendation(Recommendation),
    Service(Service),
    SpecialAnnouncement(SpecialAnnouncement),
}
