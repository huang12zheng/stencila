use crate::prelude::*;

use super::aggregate_offer::AggregateOffer;
use super::creative_work::CreativeWork;
use super::educational_occupational_program::EducationalOccupationalProgram;
use super::event::Event;
use super::menu_item::MenuItem;
use super::product::Product;
use super::service::Service;
use super::trip::Trip;

/// An offer to provide this item&#x2014;for example, an offer to sell a product, rent the DVD of a movie, perform a service, or give away tickets to an event. Use <a class="localLink" href="/businessFunction">businessFunction</a> to indicate the kind of transaction offered, i.e. sell, lease, etc. This property can also be used to describe a <a class="localLink" href="/Demand">Demand</a>. While this property is listed as expected on a number of common types, it can be used in others. In that case, using a second type, such as Product or a subtype of Product, can clarify the nature of the offer.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum offers {
    AggregateOffer(AggregateOffer),
    CreativeWork(CreativeWork),
    EducationalOccupationalProgram(EducationalOccupationalProgram),
    Event(Event),
    MenuItem(MenuItem),
    Product(Product),
    Service(Service),
    Trip(Trip),
}
