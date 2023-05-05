use crate::prelude::*;

use super::civic_structure::CivicStructure;
use super::local_business::LocalBusiness;


/// http://schema.org/announcementLocation
/// Indicates a specific <a class="localLink" href="/CivicStructure">CivicStructure</a> or <a class="localLink" href="/LocalBusiness">LocalBusiness</a> associated with the SpecialAnnouncement. For example, a specific testing facility or business with special opening hours. For a larger geographic region like a quarantine of an entire region, use <a class="localLink" href="/spatialCoverage">spatialCoverage</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum AnnouncementLocationPropEnum {
    CivicStructure(CivicStructure),
    LocalBusiness(LocalBusiness),
}
