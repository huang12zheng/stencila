use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::user_comments::UserComments;

/// The creator/author of this CreativeWork. This is the same as the Author property for CreativeWork.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum creator {
    CreativeWork(CreativeWork),
    UserComments(UserComments),
}
