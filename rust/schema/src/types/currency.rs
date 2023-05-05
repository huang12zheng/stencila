use crate::prelude::*;

use super::dated_money_specification::DatedMoneySpecification;
use super::exchange_rate_specification::ExchangeRateSpecification;
use super::loan_or_credit::LoanOrCredit;
use super::monetary_amount::MonetaryAmount;
use super::monetary_amount_distribution::MonetaryAmountDistribution;

/// The currency in which the monetary amount is expressed.<br/><br/>  Use standard formats: <a href="http://en.wikipedia.org/wiki/ISO_4217">ISO 4217 currency format</a>, e.g. "USD"; <a href="https://en.wikipedia.org/wiki/List_of_cryptocurrencies">Ticker symbol</a> for cryptocurrencies, e.g. "BTC"; well known names for <a href="https://en.wikipedia.org/wiki/Local_exchange_trading_system">Local Exchange Trading Systems</a> (LETS) and other currency types, e.g. "Ithaca HOUR".
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum currency {
    DatedMoneySpecification(DatedMoneySpecification),
    ExchangeRateSpecification(ExchangeRateSpecification),
    LoanOrCredit(LoanOrCredit),
    MonetaryAmount(MonetaryAmount),
    MonetaryAmountDistribution(MonetaryAmountDistribution),
}
