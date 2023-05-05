use crate::prelude::*;

use super::article::Article;
use super::url::URL;


/// http://schema.org/diversityStaffingReport
/// For an <a class="localLink" href="/Organization">Organization</a> (often but not necessarily a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a>), a report on staffing diversity issues. In a news context this might be for example ASNE or RTDNA (US) reports, or self-reported.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum DiversityStaffingReportPropEnum {
    Article(Article),
    URL(URL),
}
