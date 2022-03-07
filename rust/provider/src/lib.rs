use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use events::{subscribe, Subscriber};
use eyre::{bail, Result};
use node_address::Address;
use node_pointer::{walk, Visitor};
use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::str::FromStr;
use stencila_schema::{InlineContent, Node};
use tokio::{
    sync::mpsc,
    time::{timeout, Duration},
};

// Export and re-export for the convenience of crates that implement a provider
pub use ::async_trait;
pub use ::codecs;
pub use ::eyre;
pub use ::http_utils;
pub use ::once_cell;
pub use ::regex;
pub use ::stencila_schema;
pub use ::tokio;
pub use ::tracing;

pub const IMPORT: &str = "import";
pub const EXPORT: &str = "export";
pub const IMPORT_EXPORT: &str = "import/export";
pub const ACTIONS: &[&str] = &[IMPORT, EXPORT, IMPORT_EXPORT];

/// A specification for providers
///
/// All providers, including those implemented in plugins, should provide this
/// specification. Rust implementations return a `Provider` instance from the
/// `spec` function of `ProviderTrait`. Plugins provide a JSON or YAML serialization
/// as part of their manifest.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Provider {
    /// The name of the provider
    pub name: String,
}

impl Provider {
    pub fn new(name: impl AsRef<str>) -> Self {
        Self {
            name: name.as_ref().to_string(),
        }
    }
}

/// A trait for providers
///
/// This trait can be used by Rust implementations of providers, allowing them to
/// be compiled into the Stencila binaries.
#[async_trait]
pub trait ProviderTrait {
    /// Get the [`Provider`] specification
    fn spec() -> Provider;

    /// Parse a string into a node
    fn parse(_string: &str) -> Vec<ParseItem> {
        Vec::new()
    }

    /// Detect nodes within a root node that the provider may be able to identify and enrich.
    ///
    /// Returns a vector of [`Detection`].
    async fn detect(root: &Node) -> Result<Vec<DetectItem>> {
        let name = Self::spec().name;
        let parse = Box::new(|string: &str| Self::parse(string));

        let mut detector = Detector::new(name, parse);
        walk(root, &mut detector);
        Ok(detector.detections)
    }

    /// Identify a node
    ///
    /// The node is supplied to the provider, with one or more properties populated.
    /// The provider then attempts to identify the node based on those properties,
    /// and if it was able to do so, returns a copy of the node with one or more identifying
    /// properties populated (e.g. the `GithubProvider` might populate the `codeRepository` property
    /// of a `SofwareSourceCode` node).
    async fn identify(node: Node) -> Result<Node> {
        Ok(node)
    }

    /// Enrich a node
    ///
    /// If the provider had previously identified the node, then the relevant identifiers
    /// will be used to fetch enrichment data, otherwise `identify` will be called.
    /// Then, the provider will return a opy of the node with properties that are missing.
    async fn enrich(node: Node, _options: Option<EnrichOptions>) -> Result<Node> {
        Ok(node)
    }

    /// Import content from a remote [`Node`] (e.g. an `Article` or `SoftwareSourceCode` repository) to a local path
    async fn import(_node: &Node, _path: &Path, _options: Option<ImportOptions>) -> Result<bool> {
        Ok(false)
    }

    /// Export content from a local path to a remote [`Node`] (e.g. an `Article` or `SoftwareSourceCode` repository)
    async fn export(_node: &Node, _path: &Path, _options: Option<ExportOptions>) -> Result<bool> {
        Ok(false)
    }

    /// Synchronize changes between a remote [`Node`] (e.g. a `SoftwareSourceCode` repository) and a local path (a file or directory)
    async fn sync(_node: &Node, _path: &Path, _options: Option<SyncOptions>) -> Result<bool> {
        Ok(false)
    }

    /// Schedule import and/or export to/from a remove [`Node`] and a local path
    async fn schedule(_action: &str, _schedule: &str, _node: &Node, _path: &Path) -> Result<bool> {
        Ok(false)
    }
}

#[derive(Debug, Default, Clone)]
pub struct EnrichOptions {
    pub token: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct ImportOptions {
    pub token: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct ExportOptions {
    pub token: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct SyncOptions {
    pub token: Option<String>,

    /// The URL to listen on
    pub url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ParseItem {
    /// The start position in the string that the node was parsed from
    pub begin: usize,

    /// The end position in the string that the node was parsed from
    pub end: usize,

    /// The parsed [`Node`] usually with some properties populated
    pub node: Node,
}

#[derive(Debug, Serialize)]
pub struct DetectItem {
    /// The name of the provider that detected the node
    pub provider: String,

    /// The percent confidence in the detection (0-100)
    pub confidence: u32,

    /// The [`Address`], within the node tree, that the node detected node begins
    pub begin: Address,

    /// The [`Address`], within the node tree, that the node detected node ends
    pub end: Address,

    /// The detected [`Node`] usually with some properties populated (i.e. those
    /// properties that were used to detect it)
    pub node: Node,
}

pub struct Detector {
    /// The name of the provider that this detector is for
    provider: String,

    /// The function used to attempt to parse a string into a node
    parse: Box<dyn Fn(&str) -> Vec<ParseItem>>,

    /// The list of detected nodes and their location
    detections: Vec<DetectItem>,
}

impl Detector {
    fn new(provider: String, parse: Box<dyn Fn(&str) -> Vec<ParseItem>>) -> Self {
        Self {
            provider,
            parse,
            detections: Vec::new(),
        }
    }

    fn visit_string(&mut self, address: &Address, string: &str) {
        let nodes = (self.parse)(string);
        let mut detections = nodes
            .into_iter()
            .map(|ParseItem { begin, end, node }| DetectItem {
                provider: self.provider.clone(),
                confidence: 100,
                begin: address.add_index(begin),
                end: address.add_index(end),
                node,
            })
            .collect();
        self.detections.append(&mut detections);
    }
}

impl Visitor for Detector {
    fn visit_node(&mut self, address: &Address, node: &Node) -> bool {
        if let Node::String(string) = node {
            self.visit_string(address, string);
            false
        } else {
            true
        }
    }

    fn visit_inline(&mut self, address: &Address, node: &InlineContent) -> bool {
        if let InlineContent::String(string) = node {
            self.visit_string(address, string);
            false
        } else {
            true
        }
    }
}

/// Schedule import and/or export to/from a remove [`Node`] and a local path
pub async fn run_schedule(schedule: &str, sender: mpsc::Sender<()>) -> Result<()> {
    let (schedules, timezone) = cron_utils::parse(schedule)?;

    let (interrupt_sender, mut interrupt_receiver) = mpsc::unbounded_channel();
    subscribe("interrupt", Subscriber::Sender(interrupt_sender))?;

    tokio::spawn(async move {
        let interval = Duration::from_secs(1);
        let mut next = cron_utils::next(&schedules, &timezone);
        if let Some(time) = next {
            tracing::debug!("First action scheduled for {}", time);
        }
        loop {
            if let Err(..) = timeout(interval, interrupt_receiver.recv()).await {
                match next {
                    Some(time) => {
                        if Utc::now() >= time {
                            if let Err(error) = sender.send(()).await {
                                tracing::error!("When sending schedule message: {}", error);
                            }
                            next = cron_utils::next(&schedules, &timezone);
                            if let Some(time) = next {
                                tracing::debug!("Next action scheduled for {}", time);
                            }
                        }
                    }
                    None => {
                        tracing::info!("No more scheduled actions");
                        break;
                    }
                }
            } else {
                tracing::info!("Schedule was cancelled");
                break;
            }
        }
    })
    .await?;

    Ok(())
}
