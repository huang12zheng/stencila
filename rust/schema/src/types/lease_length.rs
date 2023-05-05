use crate::prelude::*;

use super::accommodation::Accommodation;
use super::offer::Offer;
use super::real_estate_listing::RealEstateListing;

/// Length of the lease for some <a class="localLink" href="/Accommodation">Accommodation</a>, either particular to some <a class="localLink" href="/Offer">Offer</a> or in some cases intrinsic to the property.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum leaseLength {
    Accommodation(Accommodation),
    Offer(Offer),
    RealEstateListing(RealEstateListing),
}
