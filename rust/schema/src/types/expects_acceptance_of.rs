use crate::prelude::*;

use super::action_access_specification::ActionAccessSpecification;
use super::consume_action::ConsumeAction;
use super::media_subscription::MediaSubscription;

/// An Offer which must be accepted before the user can perform the Action. For example, the user may need to buy a movie before being able to watch it.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum expectsAcceptanceOf {
    ActionAccessSpecification(ActionAccessSpecification),
    ConsumeAction(ConsumeAction),
    MediaSubscription(MediaSubscription),
}
