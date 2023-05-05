use crate::prelude::*;

use super::boolean::Boolean;
use super::media_subscription::MediaSubscription;

/// Indicates if use of the media require a subscription  (either paid or free). Allowed values are <code>true</code> or <code>false</code> (note that an earlier version had 'yes', 'no').
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum requiresSubscription {
    Boolean(Boolean),
    MediaSubscription(MediaSubscription),
}
