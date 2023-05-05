use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::url::URL;

/// Disclosure about verification and fact-checking processes for a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a> or other fact-checking <a class="localLink" href="/Organization">Organization</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum verificationFactCheckingPolicy {
    CreativeWork(CreativeWork),
    URL(URL),
}
