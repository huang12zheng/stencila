use crate::prelude::*;

use super::action_access_specification::ActionAccessSpecification;
use super::media_object::MediaObject;

/// Indicates if use of the media require a subscription  (either paid or free). Allowed values are <code>true</code> or <code>false</code> (note that an earlier version had 'yes', 'no').
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum requiresSubscription {
    ActionAccessSpecification(ActionAccessSpecification),
    MediaObject(MediaObject),
}
