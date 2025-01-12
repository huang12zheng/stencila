use crate::prelude::*;

use super::audio_object::AudioObject;
use super::boolean::Boolean;
use super::button::Button;
use super::cite::Cite;
use super::cite_group::CiteGroup;
use super::code_expression::CodeExpression;
use super::code_fragment::CodeFragment;
use super::date::Date;
use super::date_time::DateTime;
use super::duration::Duration;
use super::emphasis::Emphasis;
use super::image_object::ImageObject;
use super::integer::Integer;
use super::link::Link;
use super::math_fragment::MathFragment;
use super::note::Note;
use super::null::Null;
use super::number::Number;
use super::parameter::Parameter;
use super::quote::Quote;
use super::span::Span;
use super::strikeout::Strikeout;
use super::string::String;
use super::strong::Strong;
use super::subscript::Subscript;
use super::superscript::Superscript;
use super::text::Text;
use super::time::Time;
use super::timestamp::Timestamp;
use super::underline::Underline;
use super::video_object::VideoObject;

/// Union type for valid inline content.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Defaults, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]
#[def = "String(String::default())"]
pub enum Inline {
    AudioObject(AudioObject),
    Button(Button),
    Cite(Cite),
    CiteGroup(CiteGroup),
    CodeExpression(CodeExpression),
    CodeFragment(CodeFragment),
    Date(Date),
    DateTime(DateTime),
    Duration(Duration),
    Emphasis(Emphasis),
    ImageObject(ImageObject),
    Link(Link),
    MathFragment(MathFragment),
    Note(Note),
    Parameter(Parameter),
    Quote(Quote),
    Span(Span),
    Strikeout(Strikeout),
    Strong(Strong),
    Subscript(Subscript),
    Superscript(Superscript),
    Text(Text),
    Time(Time),
    Timestamp(Timestamp),
    Underline(Underline),
    VideoObject(VideoObject),
    Null(Null),
    Boolean(Boolean),
    Integer(Integer),
    Number(Number),
    String(String),
}
