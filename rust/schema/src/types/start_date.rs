use crate::prelude::*;

use super::creative_work_season::CreativeWorkSeason;
use super::creative_work_series::CreativeWorkSeries;
use super::dated_money_specification::DatedMoneySpecification;
use super::educational_occupational_program::EducationalOccupationalProgram;
use super::event::Event;
use super::merchant_return_policy_seasonal_override::MerchantReturnPolicySeasonalOverride;
use super::role::Role;
use super::schedule::Schedule;

/// The start date and time of the item (in <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO 8601 date format</a>).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum startDate {
    CreativeWorkSeason(CreativeWorkSeason),
    CreativeWorkSeries(CreativeWorkSeries),
    DatedMoneySpecification(DatedMoneySpecification),
    EducationalOccupationalProgram(EducationalOccupationalProgram),
    Event(Event),
    MerchantReturnPolicySeasonalOverride(MerchantReturnPolicySeasonalOverride),
    Role(Role),
    Schedule(Schedule),
}
