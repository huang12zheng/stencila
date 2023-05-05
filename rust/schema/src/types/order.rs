// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::boolean::Boolean;
use super::broker_prop_enum::BrokerPropEnum;
use super::customer_prop_enum::CustomerPropEnum;
use super::discount_prop_enum::DiscountPropEnum;
use super::identifier_prop_enum::IdentifierPropEnum;
use super::image_prop_enum::ImagePropEnum;
use super::invoice::Invoice;
use super::main_entity_of_page_prop_enum::MainEntityOfPagePropEnum;
use super::offer::Offer;
use super::order_date_prop_enum::OrderDatePropEnum;
use super::order_status::OrderStatus;
use super::ordered_item_prop_enum::OrderedItemPropEnum;
use super::parcel_delivery::ParcelDelivery;
use super::payment_due_date_prop_enum::PaymentDueDatePropEnum;
use super::payment_method::PaymentMethod;
use super::postal_address::PostalAddress;
use super::seller_prop_enum::SellerPropEnum;
use super::subject_of_prop_enum::SubjectOfPropEnum;
use super::text::Text;
use super::url::URL;

/// https://schema.org/Order
/// * COMMENT:
/// An order is a confirmation of a transaction (a receipt), which can contain multiple line items, each represented by an Offer that has been accepted by the customer.
/// * EXTEND FROM:
/// https://schema.org/Intangible
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct Order {
    /// An additional type for the item, typically used for adding more specific types from external vocabularies in microdata syntax. This is a relationship between something and a class that the thing is in. In RDFa syntax, it is better to use the native RDFa syntax - the 'typeof' attribute - for multiple types. Schema.org tools may have only weaker understanding of extra types, in particular those defined externally.
    pub additional_type_prop_enum: Option<URL>,

    /// An alias for the item.
    pub alternate_name_prop_enum: Option<Text>,

    /// A description of the item.
    pub description_prop_enum: Option<Text>,

    /// A sub property of description. A short description of the item used to disambiguate from other, similar items. Information from other properties (in particular, name) may be necessary for the description to be useful for disambiguation.
    pub disambiguating_description_prop_enum: Option<Text>,

    /// The identifier property represents any kind of identifier for any kind of <a class="localLink" href="/Thing">Thing</a>, such as ISBNs, GTIN codes, UUIDs etc. Schema.org provides dedicated properties for representing many of these, either as textual strings or as URL (URI) links. See <a href="/docs/datamodel.html#identifierBg">background notes</a> for more details.
    pub identifier_prop_enum: Option<IdentifierPropEnum>,

    /// An image of the item. This can be a <a class="localLink" href="/URL">URL</a> or a fully described <a class="localLink" href="/ImageObject">ImageObject</a>.
    pub image_prop_enum: Option<ImagePropEnum>,

    /// Indicates a page (or other CreativeWork) for which this thing is the main entity being described. See <a href="/docs/datamodel.html#mainEntityBackground">background notes</a> for details.
    pub main_entity_of_page_prop_enum: Option<MainEntityOfPagePropEnum>,

    /// The name of the item.
    pub name_prop_enum: Option<Text>,

    /// Indicates a potential Action, which describes an idealized action in which this thing would play an 'object' role.
    pub potential_action_prop_enum: Option<Action>,

    /// URL of a reference Web page that unambiguously indicates the item's identity. E.g. the URL of the item's Wikipedia page, Wikidata entry, or official website.
    pub same_as_prop_enum: Option<URL>,

    /// A CreativeWork or Event about this Thing.
    pub subject_of_prop_enum: Option<SubjectOfPropEnum>,

    /// URL of the item.
    pub url_prop_enum: Option<URL>,

    /// The offer(s) -- e.g., product, quantity and price combinations -- included in the order.
    pub accepted_offer_prop_enum: Option<Offer>,

    /// The billing address for the order.
    pub billing_address_prop_enum: Option<PostalAddress>,

    /// An entity that arranges for an exchange between a buyer and a seller.  In most cases a broker never acquires or releases ownership of a product or service involved in an exchange.  If it is not clear whether an entity is a broker, seller, or buyer, the latter two terms are preferred.
    pub broker_prop_enum: Option<BrokerPropEnum>,

    /// A number that confirms the given order or payment has been received.
    pub confirmation_number_prop_enum: Option<Text>,

    /// Party placing the order or paying the invoice.
    pub customer_prop_enum: Option<CustomerPropEnum>,

    /// Any discount applied (to an Order).
    pub discount_prop_enum: Option<DiscountPropEnum>,

    /// Code used to redeem a discount.
    pub discount_code_prop_enum: Option<Text>,

    /// The currency of the discount.<br/><br/>  Use standard formats: <a href="http://en.wikipedia.org/wiki/ISO_4217">ISO 4217 currency format</a>, e.g. "USD"; <a href="https://en.wikipedia.org/wiki/List_of_cryptocurrencies">Ticker symbol</a> for cryptocurrencies, e.g. "BTC"; well known names for <a href="https://en.wikipedia.org/wiki/Local_exchange_trading_system">Local Exchange Trading Systems</a> (LETS) and other currency types, e.g. "Ithaca HOUR".
    pub discount_currency_prop_enum: Option<Text>,

    /// Indicates whether the offer was accepted as a gift for someone other than the buyer.
    pub is_gift_prop_enum: Option<Boolean>,

    /// Date order was placed.
    pub order_date_prop_enum: Option<OrderDatePropEnum>,

    /// The delivery of the parcel related to this order or order item.
    pub order_delivery_prop_enum: Option<ParcelDelivery>,

    /// The identifier of the transaction.
    pub order_number_prop_enum: Option<Text>,

    /// The current status of the order.
    pub order_status_prop_enum: Option<OrderStatus>,

    /// The item ordered.
    pub ordered_item_prop_enum: Option<OrderedItemPropEnum>,

    /// The order is being paid as part of the referenced Invoice.
    pub part_of_invoice_prop_enum: Option<Invoice>,

    /// The date that payment is due.
    pub payment_due_date_prop_enum: Option<PaymentDueDatePropEnum>,

    /// The name of the credit card or other method of payment for the order.
    pub payment_method_prop_enum: Option<PaymentMethod>,

    /// An identifier for the method of payment used (e.g. the last 4 digits of the credit card).
    pub payment_method_id_prop_enum: Option<Text>,

    /// The URL for sending a payment.
    pub payment_url_prop_enum: Option<URL>,

    /// An entity which offers (sells / leases / lends / loans) the services / goods.  A seller may also be a provider.
    pub seller_prop_enum: Option<SellerPropEnum>,
}

impl Order {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
