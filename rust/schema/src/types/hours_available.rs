use crate::prelude::*;

use super::contact_point::ContactPoint;
use super::location_feature_specification::LocationFeatureSpecification;
use super::service::Service;

/// The hours during which this service or contact is available.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum hoursAvailable {
    ContactPoint(ContactPoint),
    LocationFeatureSpecification(LocationFeatureSpecification),
    Service(Service),
}
