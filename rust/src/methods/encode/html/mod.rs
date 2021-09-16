use super::Options;
use eyre::Result;
use html_escape::{encode_double_quoted_attribute, encode_safe};
use serde::Serialize;
use std::any::type_name;
use stencila_schema::*;

/// Encode a `Node` to a HTML document
pub fn encode(node: &Node, options: Option<Options>) -> Result<String> {
    let html = encode_address(node, options.clone());

    let Options {
        theme, standalone, ..
    } = options.unwrap_or_default();

    let html = if standalone {
        wrap_standalone("", &theme, &html)
    } else {
        html
    };

    Ok(html)
}

/// Generate the HTML fragment for an address within a node
///
/// This function is used when translating a `Operation` (where any value of
/// the operation is a `Node` and the operation is applied to a `Node`) to a `DomOperation`
/// (where any value is either a HTML or JSON string and the operation is applied to a browser DOM).
pub fn encode_address(node: &Node, options: Option<Options>) -> String {
    let Options {
        bundle, compact, ..
    } = options.unwrap_or_default();

    let context = Context { root: node, bundle };
    let html = node.to_html("root", &context);

    if compact {
        html
    } else {
        indent(&html)
    }
}

/// Indent generated HTML
///
/// Originally based on https://gist.github.com/lwilli/14fb3178bd9adac3a64edfbc11f42e0d
fn indent(html: &str) -> String {
    use quick_xml::events::Event;
    use quick_xml::{Reader, Writer};

    let mut buf = Vec::new();

    let mut reader = Reader::from_str(html);
    reader.trim_text(true);

    let mut writer = Writer::new_with_indent(Vec::new(), b' ', 2);

    loop {
        let ev = reader.read_event(&mut buf);

        match ev {
            Ok(Event::Eof) => break,
            Ok(event) => writer.write_event(event),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
        }
        .expect("Failed to parse XML");

        buf.clear();
    }

    std::str::from_utf8(&*writer.into_inner())
        .expect("Failed to convert a slice of bytes to a string slice")
        .to_string()
}

/// Wrap generated HTML so that it is standalone
pub fn wrap_standalone(title: &str, theme: &str, html: &str) -> String {
    let title = if title.is_empty() { "Untitled" } else { &title };
    let theme = if theme.is_empty() { "stencila" } else { &theme };

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <title>{title}</title>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link
            href="https://unpkg.com/@stencila/thema/dist/themes/{theme}/styles.css"
            rel="stylesheet">
        <script
            src="https://unpkg.com/@stencila/components/dist/stencila-components/stencila-components.esm.js"
            type="module"></script>
        <script
            src="https://unpkg.com/@stencila/components/dist/stencila-components/stencila-components.js"
            nomodule=""></script>
        <style>
            .error {{
                font-family: mono;
                color: #9e0000;
                background: #ffd9d9;
            }}
            .todo {{
                font-family: mono;
                color: #9e9b00;
                background: #faf9de;
            }}
            .unsupported {{
                font-family: mono;
                color: #777;
                background: #eee;
            }}
        </style>
    </head>
    <body>
        {html}
    </body>
</html>"#,
        title = title,
        theme = theme,
        html = html
    )
}

/// The encoding context.
///
/// Used by child nodes to retrieve necessary information about the
/// parent nodes when rendering themselves.
pub struct Context<'a> {
    /// The root node being encoded
    root: &'a Node,

    /// Whether <img>, <audio> and <video> elements should use dataURIs
    bundle: bool,
}

impl<'a> Context<'a> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Context {
            root: &Node::Null,
            bundle: false,
        }
    }
}

/// Trait for encoding a node as HTML
pub trait ToHtml {
    fn to_html(&self, slot: &str, context: &Context) -> String;
}

/// Encode a HTML element
///
/// Use this function for creating HTML strings for elements.
/// This, and other functions below, us slice `concat`, rather than `format!`
/// for performance (given that HTML generation may be done on every, or nearly every, keystroke).
fn elem(name: &str, attrs: &[String], content: &str) -> String {
    [
        "<",
        name,
        if attrs.is_empty() { "" } else { " " },
        attrs.join(" ").trim(),
        ">",
        content,
        "</",
        name,
        ">",
    ]
    .concat()
}

/// Encode an "empty" HTML element
///
/// An empty (a.k.a self-closing) element has no closing tag.
/// See https://developer.mozilla.org/en-US/docs/Glossary/Empty_element
fn elem_empty(name: &str, attrs: &[String]) -> String {
    [
        "<",
        name,
        if attrs.is_empty() { "" } else { " " },
        attrs.join(" ").trim(),
        "/>",
    ]
    .concat()
}

/// Encode a HTML element attribute, ensuring that the value is escaped correctly
fn attr(name: &str, value: &str) -> String {
    [
        name,
        "=\"",
        encode_double_quoted_attribute(&value).as_ref(),
        "\"",
    ]
    .concat()
}

/// Encode a "slot" attribute of an HTML element
fn attr_slot(slot: &str) -> String {
    attr("slot", slot)
}

/// Encode the "itemtype" attribute of an HTML element
///
/// Note: there should always be a sibling "itemscope" attribute on the
/// element.
fn attr_itemtype_string(name: &str) -> String {
    let itemtype = match name {
        // TODO: complete list of schema.org types
        "Article" | "AudioObject" | "ImageObject" | "VideoObject" => {
            ["https://schema.org/", name].concat()
        }
        _ => ["https://stenci.la/", name].concat(),
    };
    [&attr("itemtype", &itemtype), " itemscope"].concat()
}

/// Encode the "itemtype" attribute of an HTML element using the type of node
fn attr_itemtype<Type>(_value: &Type) -> String {
    let name = type_name::<Type>();
    let name = if let Some(name) = name.strip_prefix("stencila_schema::types::") {
        name
    } else {
        tracing::error!("Unexpected type: {}", name);
        name
    };
    attr_itemtype_string(name)
}

/// Encode an "itemprop" attribute of an HTML element
fn attr_itemprop(itemprop: &str) -> String {
    attr("itemprop", itemprop)
}

/// Encode a "data-itemprop" attribute of an HTML element
fn attr_data_itemprop(itemprop: &str) -> String {
    attr("data-itemprop", itemprop)
}

/// Encode a node `id` as the "id" attribute of an HTML element
fn attr_id(id: &Option<Box<String>>) -> String {
    match id.as_deref() {
        Some(id) => attr("id", id),
        None => "".to_string(),
    }
}

/// Encode a node as JSON
///
/// Several of the below implementations use this, mainly as a placeholder,
/// until a complete implementation is finished. Ensures that the JSON is
/// properly escaped
fn json(node: &impl Serialize) -> String {
    encode_safe(&serde_json::to_string_pretty(node).unwrap_or_default()).to_string()
}

/// Iterate over a slice of nodes, call a string generating function on each item
/// and concatenate the strings
pub fn concat<T, F>(slice: &[T], func: F) -> String
where
    F: FnMut(&T) -> String,
{
    slice.iter().map(func).collect::<Vec<String>>().concat()
}

mod blocks;
mod inlines;
mod nodes;
mod options;
mod primitives;
#[allow(clippy::deprecated_cfg_attr)]
mod works;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        assert_json_eq,
        utils::tests::{home, skip_slow_tests, snapshot_fixtures},
    };
    use eyre::bail;
    use insta::assert_display_snapshot;
    use serde_json::json;

    /// Encode the HTML fragment fixtures
    #[cfg(feature = "decode-html")]
    #[test]
    fn html_fragments() {
        use crate::methods::decode::html::decode;

        snapshot_fixtures("fragments/html/*.html", |_path, content| {
            let decoded = decode(&content, false).unwrap();
            let encoded = encode(
                &decoded,
                Some(Options {
                    compact: false,
                    ..Default::default()
                }),
            )
            .unwrap();
            assert_display_snapshot!(encoded);
        });
    }

    /// Validate HTML against https://validator.github.io/validator/
    ///
    /// To run locally using the validator's Docker image:
    ///
    ///  docker run -it --rm -p 8888:8888 ghcr.io/validator/validator
    ///  RUN_SLOW_TESTS=1 HTML_VALIDATOR=http://localhost:8888 cargo test
    ///
    /// See https://github.com/validator/validator/wiki/Service-%C2%BB-Input-%C2%BB-POST-body
    /// for more on the API.
    #[cfg(all(feature = "reqwest", feature = "decode-html"))]
    #[tokio::test]
    async fn nu_validate() -> Result<()> {
        use crate::methods::decode::html::decode;

        if skip_slow_tests() {
            return Ok(());
        }

        // Read the existing snapshot
        // We only do this for one, kitchen sink like, snapshot.
        let html = std::fs::read_to_string(
            home().join("rust/src/methods/encode/snapshots/html_fragments@heading.html.snap"),
        )?;
        let decoded = decode(&html, false).unwrap();
        let html = encode(
            &decoded,
            Some(Options {
                standalone: true,
                compact: false,
                ..Default::default()
            }),
        )
        .unwrap();

        // Make the POST request
        let url = if let Ok(url) = std::env::var("HTML_VALIDATOR") {
            url
        } else {
            "https://validator.w3.org/nu".to_string()
        };
        let client = reqwest::Client::new();
        let response = client
            .post([&url, "?out=json"].concat())
            .header("Content-Type", "text/html; charset=UTF-8")
            .header(
                "User-Agent",
                "Stencila tests (https://github.com/stencila/stencila/)",
            )
            .body(html)
            .send()
            .await?;
        let response = match response.error_for_status() {
            Ok(response) => response,
            Err(error) => bail!(error),
        };
        let json = response.text().await?;

        // Parse the result so it's easier to read any messages
        let result: serde_json::Value = serde_json::from_str(&json)?;
        assert_json_eq!(result, json!({"messages": []}));

        Ok(())
    }
}
