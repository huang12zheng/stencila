use crate::prelude::*;

use super::boolean::Boolean;
use super::media_subscription::MediaSubscription;


/// http://schema.org/requiresSubscription
/// Indicates if use of the media require a subscription  (either paid or free). Allowed values are <code>true</code> or <code>false</code> (note that an earlier version had 'yes', 'no').
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum RequiresSubscriptionPropEnum {
    Boolean(Boolean),
    MediaSubscription(MediaSubscription),
}
