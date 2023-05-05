use crate::prelude::*;

use super::merchant_return_policy::MerchantReturnPolicy;
use super::merchant_return_policy_seasonal_override::MerchantReturnPolicySeasonalOverride;

/// Specifies an applicable return policy (from an enumeration).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum returnPolicyCategory {
    MerchantReturnPolicy(MerchantReturnPolicy),
    MerchantReturnPolicySeasonalOverride(MerchantReturnPolicySeasonalOverride),
}
