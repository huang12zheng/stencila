use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::event::Event;
use super::organization::Organization;
use super::place::Place;
use super::product::Product;

/// Keywords or tags used to describe some item. Multiple textual entries in a keywords list are typically delimited by commas, or by repeating the property.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum keywords {
    CreativeWork(CreativeWork),
    Event(Event),
    Organization(Organization),
    Place(Place),
    Product(Product),
}
