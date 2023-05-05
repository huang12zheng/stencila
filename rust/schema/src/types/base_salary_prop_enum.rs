use crate::prelude::*;

use super::monetary_amount::MonetaryAmount;
use super::number::Number;
use super::price_specification::PriceSpecification;


/// http://schema.org/baseSalary
/// The base salary of the job or of an employee in an EmployeeRole.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum BaseSalaryPropEnum {
    MonetaryAmount(MonetaryAmount),
    Number(Number),
    PriceSpecification(PriceSpecification),
}
