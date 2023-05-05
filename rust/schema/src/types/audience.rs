use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::event::Event;
use super::lodging_business::LodgingBusiness;
use super::play_action::PlayAction;
use super::product::Product;
use super::service::Service;

/// An intended audience, i.e. a group for whom something was created.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum audience {
    CreativeWork(CreativeWork),
    Event(Event),
    LodgingBusiness(LodgingBusiness),
    PlayAction(PlayAction),
    Product(Product),
    Service(Service),
}
