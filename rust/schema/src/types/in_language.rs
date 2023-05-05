use crate::prelude::*;

use super::broadcast_service::BroadcastService;
use super::communicate_action::CommunicateAction;
use super::creative_work::CreativeWork;
use super::event::Event;
use super::link_role::LinkRole;
use super::pronounceable_text::PronounceableText;
use super::write_action::WriteAction;

/// The language of the content or performance or used in an action. Please use one of the language codes from the <a href="http://tools.ietf.org/html/bcp47">IETF BCP 47 standard</a>. See also <a class="localLink" href="/availableLanguage">availableLanguage</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum inLanguage {
    BroadcastService(BroadcastService),
    CommunicateAction(CommunicateAction),
    CreativeWork(CreativeWork),
    Event(Event),
    LinkRole(LinkRole),
    PronounceableText(PronounceableText),
    WriteAction(WriteAction),
}
