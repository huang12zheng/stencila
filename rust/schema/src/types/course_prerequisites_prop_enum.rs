use crate::prelude::*;

use super::alignment_object::AlignmentObject;
use super::course::Course;
use super::text::Text;


/// http://schema.org/coursePrerequisites
/// Requirements for taking the Course. May be completion of another <a class="localLink" href="/Course">Course</a> or a textual description like "permission of instructor". Requirements may be a pre-requisite competency, referenced using <a class="localLink" href="/AlignmentObject">AlignmentObject</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum CoursePrerequisitesPropEnum {
    AlignmentObject(AlignmentObject),
    Course(Course),
    Text(Text),
}
