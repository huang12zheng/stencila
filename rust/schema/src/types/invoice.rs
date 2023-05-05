// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::thing::Thing;
use super::broker::broker;
use super::category::category;
use super::confirmation_number::confirmationNumber;
use super::customer::customer;
use super::payment_due_date::paymentDueDate;
use super::payment_method::paymentMethod;
use super::payment_method_id::paymentMethodId;
use super::provider::provider;

/// * COMMENT: A statement of the money due for goods or services; a bill. * EXTEND FROM: https://schema.org/Intangible
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct Invoice {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<InvoiceOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct InvoiceOptions {
    /// An additional type for the item, typically used for adding more specific types from external vocabularies in microdata syntax. This is a relationship between something and a class that the thing is in. In RDFa syntax, it is better to use the native RDFa syntax - the 'typeof' attribute - for multiple types. Schema.org tools may have only weaker understanding of extra types, in particular those defined externally.
    pub additional_type: Option<Thing>,

    /// An alias for the item.
    pub alternate_name: Option<Thing>,

    /// A description of the item.
    pub description: Option<Thing>,

    /// A sub property of description. A short description of the item used to disambiguate from other, similar items. Information from other properties (in particular, name) may be necessary for the description to be useful for disambiguation.
    pub disambiguating_description: Option<Thing>,

    /// The identifier property represents any kind of identifier for any kind of <a class="localLink" href="/Thing">Thing</a>, such as ISBNs, GTIN codes, UUIDs etc. Schema.org provides dedicated properties for representing many of these, either as textual strings or as URL (URI) links. See <a href="/docs/datamodel.html#identifierBg">background notes</a> for more details.
    pub identifier: Option<Thing>,

    /// An image of the item. This can be a <a class="localLink" href="/URL">URL</a> or a fully described <a class="localLink" href="/ImageObject">ImageObject</a>.
    pub image: Option<Thing>,

    /// Indicates a page (or other CreativeWork) for which this thing is the main entity being described. See <a href="/docs/datamodel.html#mainEntityBackground">background notes</a> for details.
    pub main_entity_of_page: Option<Thing>,

    /// The name of the item.
    pub name: Option<Thing>,

    /// Indicates a potential Action, which describes an idealized action in which this thing would play an 'object' role.
    pub potential_action: Option<Thing>,

    /// URL of a reference Web page that unambiguously indicates the item's identity. E.g. the URL of the item's Wikipedia page, Wikidata entry, or official website.
    pub same_as: Option<Thing>,

    /// A CreativeWork or Event about this Thing.
    pub subject_of: Option<Thing>,

    /// URL of the item.
    pub url: Option<Thing>,

    /// The identifier for the account the payment will be applied to.
    pub account_id: Option<Invoice>,

    /// The time interval used to compute the invoice.
    pub billing_period: Option<Invoice>,

    /// An entity that arranges for an exchange between a buyer and a seller.  In most cases a broker never acquires or releases ownership of a product or service involved in an exchange.  If it is not clear whether an entity is a broker, seller, or buyer, the latter two terms are preferred.
    pub broker: Option<broker>,

    /// A category for the item. Greater signs or slashes can be used to informally indicate a category hierarchy.
    pub category: Option<category>,

    /// A number that confirms the given order or payment has been received.
    pub confirmation_number: Option<confirmationNumber>,

    /// Party placing the order or paying the invoice.
    pub customer: Option<customer>,

    /// The minimum payment required at this time.
    pub minimum_payment_due: Option<Invoice>,

    /// The date that payment is due.
    pub payment_due_date: Option<paymentDueDate>,

    /// The name of the credit card or other method of payment for the order.
    pub payment_method: Option<paymentMethod>,

    /// An identifier for the method of payment used (e.g. the last 4 digits of the credit card).
    pub payment_method_id: Option<paymentMethodId>,

    /// The status of payment; whether the invoice has been paid or not.
    pub payment_status: Option<Invoice>,

    /// The service provider, service operator, or service performer; the goods producer. Another party (a seller) may offer those services or goods on behalf of the provider. A provider may also serve as the seller.
    pub provider: Option<provider>,

    /// The Order(s) related to this Invoice. One or more Orders may be combined into a single Invoice.
    pub references_order: Option<Invoice>,

    /// The date the invoice is scheduled to be paid.
    pub scheduled_payment_date: Option<Invoice>,

    /// The total amount due.
    pub total_payment_due: Option<Invoice>,
}

impl Invoice {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
