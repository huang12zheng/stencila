use crate::prelude::*;

use super::brand::Brand;
use super::creative_work::CreativeWork;
use super::event::Event;
use super::offer::Offer;
use super::organization::Organization;
use super::place::Place;
use super::product::Product;
use super::service::Service;

/// The overall rating, based on a collection of reviews or ratings, of the item.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum aggregateRating {
    Brand(Brand),
    CreativeWork(CreativeWork),
    Event(Event),
    Offer(Offer),
    Organization(Organization),
    Place(Place),
    Product(Product),
    Service(Service),
}
