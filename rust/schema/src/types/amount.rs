use crate::prelude::*;

use super::dated_money_specification::DatedMoneySpecification;
use super::investment_or_deposit::InvestmentOrDeposit;
use super::loan_or_credit::LoanOrCredit;
use super::monetary_grant::MonetaryGrant;
use super::money_transfer::MoneyTransfer;

/// The amount of money.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum amount {
    DatedMoneySpecification(DatedMoneySpecification),
    InvestmentOrDeposit(InvestmentOrDeposit),
    LoanOrCredit(LoanOrCredit),
    MonetaryGrant(MonetaryGrant),
    MoneyTransfer(MoneyTransfer),
}
