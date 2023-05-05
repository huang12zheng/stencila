use crate::prelude::*;

use super::cdcpmd_record::CDCPMDRecord;
use super::dataset::Dataset;


/// http://schema.org/healthcareReportingData
/// Indicates data describing a hospital, e.g. a CDC <a class="localLink" href="/CDCPMDRecord">CDCPMDRecord</a> or as some kind of <a class="localLink" href="/Dataset">Dataset</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum HealthcareReportingDataPropEnum {
    CDCPMDRecord(CDCPMDRecord),
    Dataset(Dataset),
}
