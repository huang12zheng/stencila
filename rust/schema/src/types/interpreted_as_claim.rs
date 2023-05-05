use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::media_object::MediaObject;

/// Used to indicate a specific claim contained, implied, translated or refined from the content of a <a class="localLink" href="/MediaObject">MediaObject</a> or other <a class="localLink" href="/CreativeWork">CreativeWork</a>. The interpreting party can be indicated using <a class="localLink" href="/claimInterpreter">claimInterpreter</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum interpretedAsClaim {
    CreativeWork(CreativeWork),
    MediaObject(MediaObject),
}
