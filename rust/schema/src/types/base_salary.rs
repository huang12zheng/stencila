use crate::prelude::*;

use super::monetary_amount::MonetaryAmount;
use super::number::Number;
use super::price_specification::PriceSpecification;

/// The base salary of the job or of an employee in an EmployeeRole.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum baseSalary {
    MonetaryAmount(MonetaryAmount),
    Number(Number),
    PriceSpecification(PriceSpecification),
}
