use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::url::URL;


/// http://schema.org/verificationFactCheckingPolicy
/// Disclosure about verification and fact-checking processes for a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a> or other fact-checking <a class="localLink" href="/Organization">Organization</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum VerificationFactCheckingPolicyPropEnum {
    CreativeWork(CreativeWork),
    URL(URL),
}
