use std::{
    env,
    fs::remove_file,
    path::{Path, PathBuf},
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use google_drive::{self, types::Channel, DEFAULT_HOST};
use provider::{
    async_trait::async_trait,
    chrono::{DateTime, Utc},
    eyre::{bail, eyre, Result},
    futures::future,
    http_utils::{download_with, headers},
    once_cell::sync::Lazy,
    regex::Regex,
    stencila_schema::{CreativeWork, Node},
    strum::{AsRefStr, EnumString},
    tokens::token_for_provider,
    tokio::{self, sync::mpsc},
    tracing, ImportOptions, ParseItem, Provider, ProviderTrait, SyncMode, SyncOptions,
};
use server_utils::{
    axum::{
        http::{header::HeaderMap, StatusCode},
        response::Headers,
        routing, Router,
    },
    hostname, serve_gracefully,
};

/// The default name for the secret used to authenticate with the API
const SECRET_NAME: &str = "GOOGLE_TOKEN";

/// Port for the webhook server
///
/// This should not clash with any other port numbers for other providers.
/// Changes should be avoided as network configurations, such as firewall
/// rules, may assume this number.
const WEBHOOK_PORT: u16 = 10001;

// Regex targeting short identifiers
static SIMPLE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"gdrive:(folder|file|doc|sheet)/([a-zA-Z0-9-_]+)").expect("Unable to create regex")
});

// Regex targeting URL copied from the browser address bar
static URL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
                r"(?:https?://)?(?:drive|docs)\.google\.com/(drive|file|document|spreadsheets)(?:.*?)/(?:folders|d)/([a-zA-Z0-9-_]+)(?:/[^\s]*)?",
            )
            .expect("Unable to create regex")
});

/// The kind of entity on Google Drive
///
/// Used to determine which API requests to make, default import formats etc
///
/// Note: the final serialization variants below allow parsing from the URL_REGEX above
#[derive(Debug, Clone, AsRefStr, EnumString)]
#[strum(serialize_all = "lowercase", crate = "provider::strum")]
enum FileKind {
    #[strum(serialize = "folder", serialize = "drive")]
    Folder,
    File,
    #[strum(serialize = "doc", serialize = "document")]
    Doc,
    #[strum(serialize = "sheet", serialize = "spreadsheets")]
    Sheet,
}

#[derive(Clone)]
struct GoogleDriveClient {
    /// The name of the secret containing the access token
    secret_name: String,
}

impl GoogleDriveClient {
    /// Create a new Goole Drive API client
    fn new(secret_name: Option<String>) -> Self {
        let secret_name = secret_name.unwrap_or_else(|| SECRET_NAME.to_string());
        Self { secret_name }
    }

    /// Get an API token from the environment or Stencila API
    async fn token(&self) -> Result<String> {
        match env::var(&self.secret_name) {
            Ok(token) => Ok(token),
            Err(..) => match token_for_provider("google").await? {
                Some(token) => Ok(token),
                None => bail!("An access token is required to access the Google Drive API"),
            },
        }
    }

    /// Get a `google_drive` API client
    async fn api(&self) -> Result<google_drive::Client> {
        let token = self.token().await?;
        Ok(google_drive::Client::new(
            String::new(),
            String::new(),
            String::new(),
            token,
            String::new(),
        ))
    }

    /// Get additional headers required for a request
    async fn headers(&self) -> Result<Vec<(headers::HeaderName, String)>> {
        let token = self.token().await?;
        Ok(vec![(headers::AUTHORIZATION, ["Bearer ", &token].concat())])
    }

    /// Download a file/folder
    ///
    /// The `google_drive` crate provides a `files().export()` method. However, for some reason (probably
    /// because it is autogenerated from the OpenAPI spec) it returns void. So instead, we use the
    /// `http_utils::download_with` function which saves direct to the path for us too.
    async fn download(&self, file_kind: &FileKind, file_id: &str, dest: &Path) -> Result<()> {
        let headers = self.headers().await?;

        if matches!(file_kind, FileKind::Folder) {
            // Get all the children of the folder
            // See https://developers.google.com/drive/api/v3/search-files
            let query = format!("'{file_id}' in parents");
            let children = self
                .api()
                .await?
                .files()
                .list_all("", "", false, "", false, "", &query, "", false, false, "")
                .await
                .map_err(|error| eyre!(error))?;

            // Download in parallel
            let futures = children.into_iter().map(|child| {
                let dest_clone = dest.to_path_buf();
                let headers_clone = headers.clone();
                tokio::spawn(async move {
                    let file_id = child.id;
                    // TODO mime type
                    let url = format!("{DEFAULT_HOST}/files/{file_id}/export?mimeType=text/html");
                    let filename = PathBuf::from(child.name);
                    let path = dest_clone.join(filename);
                    tracing::debug!(
                        "Downloading child `{file_id}` to `{path}`",
                        path = path.display()
                    );
                    download_with(&url, &path, &headers_clone).await
                })
            });
            let results = future::join_all(futures).await;
            for result in results {
                result??;
            }
        } else {
            // The destination path does not have an extension so use the default for the kind
            // The list of available export formats: https://developers.google.com/drive/api/v3/ref-export-formats
            let mime_type = match file_kind {
                FileKind::Doc => "text/html",
                FileKind::Sheet => "text/csv",
                _ => "",
            };

            let url = format!("{DEFAULT_HOST}/files/{file_id}/export?mimeType={mime_type}");
            download_with(&url, dest, &headers).await?;
        }

        Ok(())
    }
}

pub struct GoogleDriveProvider;

impl GoogleDriveProvider {
    /// Create an URL for a Google Drive resource (usually to store in a [`CreativeWork`] node)
    fn create_url(kind: &FileKind, id: &str) -> String {
        let prefix = match kind {
            FileKind::Folder => "https://drive.google.com/drive/folders/",
            FileKind::File => "https://drive.google.com/file/d/",
            FileKind::Doc => "https://docs.google.com/document/d/",
            FileKind::Sheet => "https://docs.google.com/spreadsheets/d/",
        };
        [prefix, id].concat()
    }

    /// Parse a URL for a Google Drive resources (usually to retrieve the kind and id from the URL of a [`CreativeWork`])
    fn parse_url(url: &str) -> Result<(FileKind, String)> {
        let captures = URL_REGEX
            .captures(url)
            .ok_or_else(|| eyre!("Unable to parse as a Google Drive URL"))?;
        let kind = FileKind::from_str(&captures[1])?;
        let id = captures[2].to_string();
        Ok((kind, id))
    }

    /// Extract the kind and id of a Google Drive resource from the URL of a [`CreativeWork`]
    fn file_kind_id(node: &Node) -> Result<(FileKind, String)> {
        let work = match node {
            Node::CreativeWork(work) => work,
            _ => bail!("Unrecognized node type"),
        };
        match &work.url {
            Some(url) => Self::parse_url(url),
            None => bail!("Creative work has no `url` property"),
        }
    }
}

#[async_trait]
impl ProviderTrait for GoogleDriveProvider {
    fn spec() -> Provider {
        Provider::new("gdrive")
    }

    fn parse(string: &str) -> Vec<ParseItem> {
        SIMPLE_REGEX
            .captures_iter(string)
            .into_iter()
            .map(|captures| {
                let capture = captures.get(0).unwrap();
                (
                    capture.start(),
                    capture.end(),
                    FileKind::from_str(&captures[1]).unwrap_or(FileKind::File),
                    captures[2].to_string(),
                )
            })
            .chain(URL_REGEX.captures_iter(string).into_iter().map(|captures| {
                let capture = captures.get(0).unwrap();
                (
                    capture.start(),
                    capture.end(),
                    FileKind::from_str(&captures[1]).unwrap_or(FileKind::File),
                    captures[2].to_string(),
                )
            }))
            .map(|(begin, end, kind, id)| ParseItem {
                begin,
                end,
                node: Node::CreativeWork(CreativeWork {
                    url: Some(Box::new(Self::create_url(&kind, &id))),
                    ..Default::default()
                }),
            })
            .collect()
    }

    fn recognize(node: &Node) -> bool {
        Self::file_kind_id(node).is_ok()
    }

    async fn import(node: &Node, dest: &Path, options: Option<ImportOptions>) -> Result<()> {
        let (file_kind, file_id) = Self::file_kind_id(node)?;

        let options = options.unwrap_or_default();
        let client = GoogleDriveClient::new(options.secret_name);

        tracing::info!("Downloading {}", Self::create_url(&file_kind, &file_id));
        client.download(&file_kind, &file_id, dest).await
    }

    /// Synchronize from a Google Drive resource to a local file or folder
    ///
    /// This does not yet handle syncing of folders. For those we will probably need to set up
    /// individual watches for each file in the folder. Note that the the user still needs to give
    /// Stencila permission to access each individual file in a folder.
    async fn sync(
        node: &Node,
        dest: &Path,
        _canceller: mpsc::Receiver<()>,
        options: Option<SyncOptions>,
    ) -> Result<()> {
        let (file_kind, file_id) = Self::file_kind_id(node)?;
        let file_url = Self::create_url(&file_kind, &file_id);

        let options = options.unwrap_or_default();
        let client = GoogleDriveClient::new(options.secret_name);

        // See https://developers.google.com/drive/api/v3/push for docs related to this

        // Generate the unique id for the webhook channel
        let channel_id = uuids::generate_num("wh", 36).to_string();

        // Create a URL for the channel
        let channel_host = match options.host {
            Some(host) => host,
            None => format!(
                "{hostname}:{port}",
                hostname = hostname().await,
                port = WEBHOOK_PORT
            ),
        };
        let channel_url = format!("https://{channel_host}/{channel_id}");

        // Generate the token used for validating notifications for this channel
        let channel_token = key_utils::generate();

        // Unix timestamp (in ms) for when this should expire. The maximum is 1 day (86400s) for file resources
        // and 1 week (604800s) for changes.
        let expiration = now() + 86400 * 1000;

        tracing::info!("Creating Google Drive webhook for {}", file_url);
        let channel = client
            .api()
            .await?
            .files()
            .watch(
                &file_id,
                false,
                "",
                false,
                false,
                &Channel {
                    id: channel_id.clone(),
                    token: channel_token.clone(),
                    address: channel_url,
                    expiration: expiration as i64,
                    // The following are fixed / optional but required by `google_drive::Channel`
                    type_: "webhook".to_string(),
                    kind: "api#channel".to_string(),
                    resource_id: "".to_string(),
                    resource_uri: "".to_string(),
                    payload: None,
                    params: "".to_string(),
                },
            )
            .await
            .map_err(|error| eyre!(error))?;

        // Listen for webhook events
        let response_headers = Headers(vec![(
            "Server",
            format!(
                "Stencila-Google-Drive-Provider/{} ({})",
                env!("CARGO_PKG_VERSION"),
                env::consts::OS
            ),
        )]);
        let dest = dest.to_path_buf();
        let sync_mode = options.mode.unwrap_or_default();
        let client_clone = client.clone();
        let mut last_revision_time = Utc::now();
        let router = Router::new().route(
            &format!("/{}", channel_id),
            routing::post(move |request_headers: HeaderMap| async move {
                match webhook_event(
                    request_headers,
                    &file_kind,
                    &file_id,
                    &dest,
                    &sync_mode,
                    &client_clone,
                    &channel_id,
                    &channel_token,
                    &mut last_revision_time,
                )
                .await
                {
                    Ok((status, msg)) => (status, response_headers, msg),
                    Err(error) => {
                        tracing::error!("While handling webhook event: {}", error);
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            response_headers,
                            "Internal server error".into(),
                        )
                    }
                }
            }),
        );
        serve_gracefully([0, 0, 0, 0], WEBHOOK_PORT, router).await?;

        // Stop the channel
        match client.api().await?.channels().stop(&channel).await {
            Ok(..) => {
                tracing::info!("Deleted Google Drive webhook for {}", file_url);
            }
            Err(error) => {
                tracing::warn!("While deleting Google Drive webhook: {}", error);
            }
        }

        Ok(())
    }
}

/// Handle Google Drive webhook events
#[allow(clippy::too_many_arguments)]
async fn webhook_event(
    headers: HeaderMap,
    file_kind: &FileKind,
    file_id: &str,
    dest: &Path,
    sync_mode: &SyncMode,
    client: &GoogleDriveClient,
    channel_id: &str,
    channel_token: &str,
    last_revision_time: &mut DateTime<Utc>,
) -> Result<(StatusCode, String)> {
    if headers
        .get("X-Goog-Channel-Id")
        .map_or("", |val| val.to_str().unwrap_or_default())
        != channel_id
    {
        // This is most likely to happen when a previously created channel for this
        // resource was not stopped (or expired).
        let msg = "Rejected notification with invalid channel id";
        tracing::warn!("{}", msg);
        return Ok((StatusCode::BAD_REQUEST, msg.into()));
    }

    if headers
        .get("X-Goog-Channel-Token")
        .map_or("", |val| val.to_str().unwrap_or_default())
        != channel_token
    {
        let msg = "Rejected notification with invalid channel token";
        tracing::warn!("{}", msg);
        return Ok((StatusCode::BAD_REQUEST, msg.into()));
    }

    let state = match headers.get("X-Goog-Resource-State") {
        Some(state) => state.to_str()?,
        None => {
            let msg = "Rejected notification without `X-Goog-Resource-State`";
            tracing::warn!("{}", msg);
            return Ok((StatusCode::BAD_REQUEST, msg.into()));
        }
    };

    // For a list of states see https://developers.google.com/drive/api/v3/push#understanding-drive-api-notification-events
    match state {
        "sync" => {
            // Initial sync message
            // See https://developers.google.com/drive/api/v3/push#sync-message
            let msg = format!("Channel sync notification for `{}", channel_id);
            tracing::debug!("{}", msg);
            return Ok((StatusCode::OK, msg));
        }
        "add" | "untrash" => {
            tracing::info!(
                "Google Drive file `{}` was added or untrashed, adding `{}`",
                file_id,
                dest.display()
            );
            client.download(file_kind, file_id, dest).await?;
        }
        "update" => {
            // File was updated so get the type of change (so we can ignore changes to `parents`
            // `children` and `permissions`; at least for now)
            let changed = headers
                .get("X-Goog-Changed")
                .map_or("", |val| val.to_str().unwrap_or_default());
            if changed.contains("content") || changed.contains("properties") {
                match sync_mode {
                    SyncMode::Live => {
                        // Change in content so download
                        // Experiments showed that notifications for changes to content
                        // are throttled to once every three minutes (at least for sheets). So "live"
                        // means with up to three minute delay (but will be immediate if the last change
                        // was more than three minutes ago)
                        tracing::info!(
                            "Google Drive file `{}` changed, updating `{}`",
                            file_id,
                            dest.display()
                        );
                        client.download(file_kind, file_id, dest).await?;
                    }
                    SyncMode::Tagged => {
                        // See if there is a new revision
                        if let Some(revision) = client
                            .api()
                            .await?
                            .revisions()
                            .list_all(file_id)
                            .await
                            .map_err(|error| eyre!(error))?
                            .last()
                        {
                            if let Some(time) = revision.modified_time {
                                if time > *last_revision_time {
                                    tracing::info!(
                                        "Google Drive file `{}` has a new revision, updating `{}`",
                                        file_id,
                                        dest.display()
                                    );
                                    client.download(file_kind, file_id, dest).await?;
                                    *last_revision_time = time;
                                }
                            }
                        }
                    }
                }
            }
        }
        "remove" | "trash" => {
            tracing::info!(
                "Google Drive file `{}` was removed or trashed, deleting `{}`",
                file_id,
                dest.display()
            );
            if dest.exists() {
                remove_file(dest)?;
            }
        }
        _ => {
            let msg = format!("Ignoring notification for state `{}`", state);
            tracing::warn!("{}", msg);
            return Ok((StatusCode::ACCEPTED, msg));
        }
    }

    let msg = "Webhook event handled";
    tracing::info!("{}", msg);
    Ok((StatusCode::OK, msg.into()))
}

/// Get the current timestamp (in milliseconds)
fn now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time should not go backwards")
        .as_millis()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::assert_json_is;

    #[test]
    fn parse() {
        for string in [
            "gdrive:folder/1OcB7VTWb3lc0u8FJX2LXc5GraKpn-r_m",
            "drive.google.com/drive/folders/1OcB7VTWb3lc0u8FJX2LXc5GraKpn-r_m",
            "https://drive.google.com/drive/folders/1OcB7VTWb3lc0u8FJX2LXc5GraKpn-r_m",
            "https://drive.google.com/drive/u/1/folders/1OcB7VTWb3lc0u8FJX2LXc5GraKpn-r_m",
        ] {
            assert_json_is!(
                GoogleDriveProvider::parse(string)[0].node,
                {
                    "type": "CreativeWork",
                    "url": "https://drive.google.com/drive/folders/1OcB7VTWb3lc0u8FJX2LXc5GraKpn-r_m",
                }
            );
        }

        for string in [
            "gdrive:file/1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA",
            "drive.google.com/file/d/1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA",
            "https://drive.google.com/file/d/1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA",
            "https://drive.google.com/file/d/1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA/view?usp=sharing",
            ] {
                assert_json_is!(
                    GoogleDriveProvider::parse(string)[0].node,
                    {
                        "type": "CreativeWork",
                        "url": "https://drive.google.com/file/d/1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA",
                    }
                );
            }

        for string in [
                "docs.google.com/document/d/1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA",
                "https://docs.google.com/document/d/1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA/",
                "https://docs.google.com/document/d/1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA/edit",
                "https://docs.google.com/document/u/1/d/1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA/edit",
            ] {
                assert_json_is!(
                    GoogleDriveProvider::parse(string)[0].node,
                    {
                        "type": "CreativeWork",
                        "url": "https://docs.google.com/document/d/1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA",
                    }
                );
            }

        for string in [
            "docs.google.com/spreadsheets/d/1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA",
            "https://docs.google.com/spreadsheets/d/1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA/",
            "https://docs.google.com/spreadsheets/d/1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA/edit",
            "https://docs.google.com/spreadsheets/u/0/d/1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA/edit",
        ] {
            assert_json_is!(
                GoogleDriveProvider::parse(string)[0].node,
                {
                    "type": "CreativeWork",
                    "url": "https://docs.google.com/spreadsheets/d/1BW6MubIyDirCGW9Wq-tSqCma8pioxBI6VpeLyXn5mZA",
                }
            );
        }

        // Multiple items in a string
        let parse_items = GoogleDriveProvider::parse(
            "
            gdrive:file/17Fw92iZgjD9dEcE8N2m08m-CRa3g-6_Ar24TLumjVV0 som word to be ignored
            and then another url https://docs.google.com/spreadsheets/d/1STkgekwd0Vqo9wj8huU2ps9RaPRvfAWDF7GoR5Vb3GY
        ",
        );
        assert_eq!(parse_items.len(), 2);
        assert_json_is!(parse_items[0].node, {
            "type": "CreativeWork",
            "url": "https://drive.google.com/file/d/17Fw92iZgjD9dEcE8N2m08m-CRa3g-6_Ar24TLumjVV0",
        });
        assert_json_is!(parse_items[1].node, {
            "type": "CreativeWork",
            "url": "https://docs.google.com/spreadsheets/d/1STkgekwd0Vqo9wj8huU2ps9RaPRvfAWDF7GoR5Vb3GY",
        });
    }
}
