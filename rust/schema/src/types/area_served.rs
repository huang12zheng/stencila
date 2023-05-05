use crate::prelude::*;

use super::contact_point::ContactPoint;
use super::delivery_charge_specification::DeliveryChargeSpecification;
use super::demand::Demand;
use super::offer::Offer;
use super::organization::Organization;
use super::service::Service;

/// The geographic area where a service or offered item is provided.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum areaServed {
    ContactPoint(ContactPoint),
    DeliveryChargeSpecification(DeliveryChargeSpecification),
    Demand(Demand),
    Offer(Offer),
    Organization(Organization),
    Service(Service),
}
