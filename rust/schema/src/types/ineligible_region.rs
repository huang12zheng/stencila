use crate::prelude::*;

use super::action_access_specification::ActionAccessSpecification;
use super::delivery_charge_specification::DeliveryChargeSpecification;
use super::demand::Demand;
use super::media_object::MediaObject;
use super::offer::Offer;

/// The ISO 3166-1 (ISO 3166-1 alpha-2) or ISO 3166-2 code, the place, or the GeoShape for the geo-political region(s) for which the offer or delivery charge specification is not valid, e.g. a region where the transaction is not allowed.<br/><br/>  See also <a class="localLink" href="/eligibleRegion">eligibleRegion</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum ineligibleRegion {
    ActionAccessSpecification(ActionAccessSpecification),
    DeliveryChargeSpecification(DeliveryChargeSpecification),
    Demand(Demand),
    MediaObject(MediaObject),
    Offer(Offer),
}
