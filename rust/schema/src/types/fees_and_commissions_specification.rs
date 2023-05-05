use crate::prelude::*;

use super::financial_product::FinancialProduct;
use super::financial_service::FinancialService;

/// Description of fees, commissions, and other terms applied either to a class of financial product, or by a financial service organization.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum feesAndCommissionsSpecification {
    FinancialProduct(FinancialProduct),
    FinancialService(FinancialService),
}
