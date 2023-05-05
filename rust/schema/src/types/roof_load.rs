use crate::prelude::*;

use super::bus_or_coach::BusOrCoach;
use super::car::Car;

/// The permitted total weight of cargo and installations (e.g. a roof rack) on top of the vehicle.<br/><br/>  Typical unit code(s): KGM for kilogram, LBR for pound<br/><br/>  <ul> <li>Note 1: You can indicate additional information in the <a class="localLink" href="/name">name</a> of the <a class="localLink" href="/QuantitativeValue">QuantitativeValue</a> node.</li> <li>Note 2: You may also link to a <a class="localLink" href="/QualitativeValue">QualitativeValue</a> node that provides additional information using <a class="localLink" href="/valueReference">valueReference</a></li> <li>Note 3: Note that you can use <a class="localLink" href="/minValue">minValue</a> and <a class="localLink" href="/maxValue">maxValue</a> to indicate ranges.</li> </ul>
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum roofLoad {
    BusOrCoach(BusOrCoach),
    Car(Car),
}
