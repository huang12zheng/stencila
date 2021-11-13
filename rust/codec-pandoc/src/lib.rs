use codec::{
    async_trait::async_trait,
    eyre::{bail, Result},
    stencila_schema::Node,
    utils::vec_string,
    Codec, CodecTrait, DecodeOptions, EncodeOptions,
};
use pandoc_types::definition as pandoc;
use std::{io::Write, path::PathBuf, process::Stdio};

#[cfg(feature = "decode")]
mod decode;

#[cfg(feature = "decode")]
pub use decode::{decode, decode_fragment, decode_pandoc};

#[cfg(feature = "encode")]
mod encode;

#[cfg(feature = "encode")]
pub use encode::{encode, encode_node};

/// A codec for Pandoc JSON
///
/// Pandoc JSON is a serialization of Pandoc's internal representation of
/// documents. It is Pandoc's equivalent of Stencila JSON.
///
/// This codec translates between Stencila nodes and Pandoc JSON, primarily
/// as the basis for other codec that are powered by Pandoc.
pub struct PandocCodec {}

#[async_trait]
impl CodecTrait for PandocCodec {
    fn spec() -> Codec {
        Codec {
            status: "alpha".to_string(),
            formats: vec_string!["pandoc"],
            root_types: vec_string!["Article"],
            from_string: cfg!(feature = "decode"),
            from_path: cfg!(feature = "decode"),
            to_string: cfg!(feature = "encode"),
            to_path: cfg!(feature = "encode"),
            unsupported_types: vec_string![
                // TODO: Fix these
                "Table",
                "AudioObject",
                "ImageObject",
                "VideoObject"
            ],
            ..Default::default()
        }
    }

    #[cfg(feature = "decode")]
    async fn from_str_async(str: &str, _options: Option<DecodeOptions>) -> Result<Node> {
        decode(str, None, "pandoc", &[]).await
    }

    #[cfg(feature = "encode")]
    async fn to_string_async(node: &Node, _options: Option<EncodeOptions>) -> Result<String> {
        encode(node, None, "pandoc", &[]).await
    }
}

/// The semver requirement for Pandoc.
///
/// Note that this is a semver *requirement*, so higher versions of Pandoc
/// that meet this, should still work.
///
/// This is partially based on compatibility with the `pandoc_types` crate.
/// Some recent changes to the pandoc-types versions used by Pandoc (from https://pandoc.org/releases.html):
///
///   pandoc 2.11 (2020-10-11) : pandoc-types 1.22
///   pandoc 2.10 (2020-06-29) : pandoc-types 1.21
pub const PANDOC_SEMVER: &str = ">=2.11";

/// Call Pandoc binary to convert some input content to Pandoc JSON.
pub async fn from_pandoc(
    input: &str,
    path: Option<PathBuf>,
    format: &str,
    args: &[&str],
) -> Result<pandoc::Pandoc> {
    let json = if format == "pandoc" {
        input.to_string()
    } else {
        let binary = binaries::require("pandoc", PANDOC_SEMVER).await?;

        let mut command = binary.command();
        command.args(["--from", format, "--to", "json"]);
        command.args(args);
        command.stdout(Stdio::piped());

        let child = if let Some(path) = path {
            if !path.exists() {
                bail!("File does not exists: {}", path.to_string_lossy())
            }
            command.arg(path).spawn()?
        } else {
            let mut child = command.stdin(Stdio::piped()).spawn()?;
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(input.as_ref())?;
            }
            child
        };

        let result = child.wait_with_output()?;
        std::str::from_utf8(result.stdout.as_ref())?.to_string()
    };

    Ok(serde_json::from_str(&json)?)
}

/// Call Pandoc binary to convert Pandoc JSON to some output format
pub async fn to_pandoc(
    doc: pandoc::Pandoc,
    path: Option<PathBuf>,
    format: &str,
    args: &[String],
) -> Result<String> {
    let json = serde_json::to_string(&doc)?;

    if format == "pandoc" {
        Ok(json)
    } else {
        let binary = binaries::require("pandoc", PANDOC_SEMVER).await?;

        let mut command = binary.command();
        command.args(["--from", "json", "--to", format]);
        command.args(args);
        if let Some(path) = &path {
            command.args(["--output", &path.to_string_lossy()]);
        }

        let mut child = command
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()?;
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(json.as_ref())?;
        }

        let result = child.wait_with_output()?;
        let stdout = std::str::from_utf8(result.stdout.as_ref())?.to_string();

        if let Some(path) = path {
            Ok(path.to_string_lossy().to_string())
        } else {
            Ok(stdout)
        }
    }
}