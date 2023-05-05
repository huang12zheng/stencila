use crate::prelude::*;

use super::permit::Permit;
use super::ticket::Ticket;

/// The organization issuing the ticket or permit.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum issuedBy {
    Permit(Permit),
    Ticket(Ticket),
}
