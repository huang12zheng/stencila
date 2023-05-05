use crate::prelude::*;

use super::alignment_object::AlignmentObject;
use super::course::Course;
use super::text::Text;

/// Requirements for taking the Course. May be completion of another <a class="localLink" href="/Course">Course</a> or a textual description like "permission of instructor". Requirements may be a pre-requisite competency, referenced using <a class="localLink" href="/AlignmentObject">AlignmentObject</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum coursePrerequisites {
    AlignmentObject(AlignmentObject),
    Course(Course),
    Text(Text),
}
