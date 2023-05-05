use crate::prelude::*;

use super::aggregate_offer::AggregateOffer;
use super::creative_work::CreativeWork;
use super::event::Event;
use super::menu_item::MenuItem;
use super::product::Product;
use super::service::Service;
use super::trip::Trip;


/// http://schema.org/itemOffered
/// An item being offered (or demanded). The transactional nature of the offer or demand is documented using <a class="localLink" href="/businessFunction">businessFunction</a>, e.g. sell, lease etc. While several common expected types are listed explicitly in this definition, others can be used. Using a second type, such as Product or a subtype of Product, can clarify the nature of the offer.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ItemOfferedPropEnum {
    AggregateOffer(AggregateOffer),
    CreativeWork(CreativeWork),
    Event(Event),
    MenuItem(MenuItem),
    Product(Product),
    Service(Service),
    Trip(Trip),
}
