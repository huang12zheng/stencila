use crate::prelude::*;

use super::demand::Demand;
use super::offer::Offer;

/// An item being offered (or demanded). The transactional nature of the offer or demand is documented using <a class="localLink" href="/businessFunction">businessFunction</a>, e.g. sell, lease etc. While several common expected types are listed explicitly in this definition, others can be used. Using a second type, such as Product or a subtype of Product, can clarify the nature of the offer.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum itemOffered {
    Demand(Demand),
    Offer(Offer),
}
