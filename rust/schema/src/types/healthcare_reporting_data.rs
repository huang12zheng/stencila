use crate::prelude::*;

use super::cdcpmd_record::CDCPMDRecord;
use super::dataset::Dataset;

/// Indicates data describing a hospital, e.g. a CDC <a class="localLink" href="/CDCPMDRecord">CDCPMDRecord</a> or as some kind of <a class="localLink" href="/Dataset">Dataset</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum healthcareReportingData {
    CDCPMDRecord(CDCPMDRecord),
    Dataset(Dataset),
}
