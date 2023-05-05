use crate::prelude::*;

use super::accommodation::Accommodation;
use super::apartment_complex::ApartmentComplex;
use super::place::Place;

/// A page providing information on how to book a tour of some <a class="localLink" href="/Place">Place</a>, such as an <a class="localLink" href="/Accommodation">Accommodation</a> or <a class="localLink" href="/ApartmentComplex">ApartmentComplex</a> in a real estate setting, as well as other kinds of tours as appropriate.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum tourBookingPage {
    Accommodation(Accommodation),
    ApartmentComplex(ApartmentComplex),
    Place(Place),
}
