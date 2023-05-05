use crate::prelude::*;

use super::cdcpmd_record::CDCPMDRecord;
use super::job_posting::JobPosting;
use super::real_estate_listing::RealEstateListing;
use super::special_announcement::SpecialAnnouncement;

/// Publication date of an online listing.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum datePosted {
    CDCPMDRecord(CDCPMDRecord),
    JobPosting(JobPosting),
    RealEstateListing(RealEstateListing),
    SpecialAnnouncement(SpecialAnnouncement),
}
