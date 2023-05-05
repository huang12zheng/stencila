use crate::prelude::*;

use super::contact_point::ContactPoint;
use super::lodging_business::LodgingBusiness;
use super::service_channel::ServiceChannel;
use super::tourist_attraction::TouristAttraction;

/// A language someone may use with or at the item, service or place. Please use one of the language codes from the <a href="http://tools.ietf.org/html/bcp47">IETF BCP 47 standard</a>. See also <a class="localLink" href="/inLanguage">inLanguage</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum availableLanguage {
    ContactPoint(ContactPoint),
    LodgingBusiness(LodgingBusiness),
    ServiceChannel(ServiceChannel),
    TouristAttraction(TouristAttraction),
}
