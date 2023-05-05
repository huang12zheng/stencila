use crate::prelude::*;

use super::merchant_return_policy::MerchantReturnPolicy;
use super::merchant_return_policy_seasonal_override::MerchantReturnPolicySeasonalOverride;

/// Specifies either a fixed return date or the number of days (from the delivery date) that a product can be returned. Used when the <a class="localLink" href="/returnPolicyCategory">returnPolicyCategory</a> property is specified as <a class="localLink" href="/MerchantReturnFiniteReturnWindow">MerchantReturnFiniteReturnWindow</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum merchantReturnDays {
    MerchantReturnPolicy(MerchantReturnPolicy),
    MerchantReturnPolicySeasonalOverride(MerchantReturnPolicySeasonalOverride),
}
