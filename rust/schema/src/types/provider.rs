use crate::prelude::*;

use super::action::Action;
use super::creative_work::CreativeWork;
use super::educational_occupational_program::EducationalOccupationalProgram;
use super::invoice::Invoice;
use super::parcel_delivery::ParcelDelivery;
use super::reservation::Reservation;
use super::service::Service;
use super::trip::Trip;

/// The service provider, service operator, or service performer; the goods producer. Another party (a seller) may offer those services or goods on behalf of the provider. A provider may also serve as the seller.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum provider {
    Action(Action),
    CreativeWork(CreativeWork),
    EducationalOccupationalProgram(EducationalOccupationalProgram),
    Invoice(Invoice),
    ParcelDelivery(ParcelDelivery),
    Reservation(Reservation),
    Service(Service),
    Trip(Trip),
}
