use std::{
    env,
    fs::{create_dir_all, remove_file, File},
    io::Write,
    path::{Path, PathBuf},
};

use archive_utils::extract_tar;
use http_utils::{
    delete_with, download_temp_with, get_with, headers, post_with,
    serde::{de::DeserializeOwned, Deserialize, Serialize},
    serde_json::json,
    tempfile::NamedTempFile,
};
use provider::{
    async_trait::async_trait,
    codecs,
    eyre::{bail, Result},
    once_cell::sync::Lazy,
    regex::Regex,
    stencila_schema::{
        CreativeWorkContent, CreativeWorkPublisher, CreativeWorkVersion, Date, Node, Organization,
        SoftwareSourceCode, ThingDescription,
    },
    tracing, EnrichOptions, ImportOptions, ParseItem, Provider, ProviderTrait, SyncOptions,
};
use server_utils::{
    axum::{
        http::{header::HeaderMap, StatusCode},
        response::Headers,
        routing, Json, Router,
    },
    serve_gracefully,
};

pub struct GitlabProvider;

/// Default port for the webhook server (it's useful to have a fixed port for testing)
const WATCH_SERVER_PORT: u16 = 1651; // python3 -c "print(1024 + sum([ord(c) for c in 'gitlab']))"

/// A client for the Gitlab REST API
///
/// Although there is an existing Rust client for the Gitlab REST API there
/// were several difficulties with using it:
///  - it is blocking and thus needs workarounds for use withing async functions
///  - it requires an access token event though the API allows some routes to be used without
///  - it does not define many types for returned payloads (so they need to be coded up anyway)
/// Also by using out existing HTTP client we benefit from caching.
#[derive(Clone)]
struct GitlabClient {
    /// The base URL
    base_url: String,

    /// Headers to include in each request
    headers: Vec<(headers::HeaderName, String)>,
}

impl GitlabClient {
    fn new(token: Option<String>) -> Result<Self> {
        let base_url = "https://gitlab.com/api/v4/".to_string();
        let headers = match token {
            Some(token) => vec![(headers::AUTHORIZATION, ["Bearer ", &token].concat())],
            None => vec![],
        };
        Ok(Self { base_url, headers })
    }

    async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        get_with(&[&self.base_url, path].concat(), &self.headers).await
    }

    async fn post<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: B) -> Result<T> {
        post_with(&[&self.base_url, path].concat(), body, &self.headers).await
    }

    async fn delete(&self, path: &str) -> Result<()> {
        delete_with(&[&self.base_url, path].concat(), &self.headers).await
    }

    async fn download_temp(&self, path: &str) -> Result<NamedTempFile> {
        download_temp_with(&[&self.base_url, path].concat(), None, &self.headers).await
    }
}

impl GitlabProvider {
    /// Extract the Gitlab org and project name from a [`SoftwareSourceCode`] node
    fn org_name(ssc: &SoftwareSourceCode) -> Option<(&str, &str)> {
        if let Some(repo) = &ssc.code_repository {
            if let Some(repo) = repo.strip_prefix("https://gitlab.com/") {
                let parts: Vec<&str> = repo.split('/').collect();
                if parts.len() >= 2 {
                    Some((parts[0], parts[1]))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Extract the Gitlab project from a [`SoftwareSourceCode`] node as a URL encoded path
    ///
    /// See https://docs.gitlab.com/ee/api/index.html#namespaced-path-encoding
    fn project_id(ssc: &SoftwareSourceCode) -> Option<String> {
        Self::org_name(ssc).map(|(org, name)| format!("{}%2F{}", org, name))
    }

    /// Extract the sub-path from a [`SoftwareSourceCode`] node (if any)
    fn path(ssc: &SoftwareSourceCode) -> Option<&str> {
        ssc.content
            .as_ref()
            .and_then(|content| match content.as_ref() {
                CreativeWorkContent::String(path) => Some(path.as_str()),
                _ => None,
            })
    }

    /// Extract the sub-path from a [`SoftwareSourceCode`] node as a URL encoded path
    fn path_id(ssc: &SoftwareSourceCode) -> Option<String> {
        Self::path(ssc).map(|path| urlencoding::encode(path).to_string())
    }

    /// Extract the version from a [`SoftwareSourceCode`] node (if any)
    fn version(ssc: &SoftwareSourceCode) -> Option<&str> {
        ssc.version
            .as_ref()
            .and_then(|version| match version.as_ref() {
                CreativeWorkVersion::String(version) => Some(version.as_str()),
                _ => None,
            })
    }
}

#[async_trait]
impl ProviderTrait for GitlabProvider {
    fn spec() -> Provider {
        Provider::new("gitlab")
    }

    fn parse(string: &str) -> Vec<ParseItem> {
        // Regex targeting short identifiers e.g. gitlab:org/name
        static SHORT_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^gitlab:([a-z0-9\-]+)/([a-z0-9\-_]+)(?:/([^@]+))?(?:@(.+))?$")
                .expect("Unable to create regex")
        });

        // Regex targeting URL copied from the browser address bar
        // Note that compared to the equivalent Gitlab URLs, these have an additional `-/` before `tree` or `blob`
        static URL_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^https?://gitlab\.com/([a-z0-9\-]+)/([a-z0-9\-_]+)/?(?:-/tree|-/blob)?/?([^/]+)?/?(.+)?$")
                .expect("Unable to create regex")
        });

        SHORT_REGEX
            .captures_iter(string)
            .into_iter()
            .map(|captures| {
                let capture = captures.get(0).unwrap();
                (
                    capture.start(),
                    capture.end(),
                    captures[1].to_string(),
                    captures[2].to_string(),
                    captures.get(4).map(|group| group.as_str().to_string()),
                    captures.get(3).map(|group| group.as_str().to_string()),
                )
            })
            .chain(URL_REGEX.captures_iter(string).into_iter().map(|captures| {
                let capture = captures.get(0).unwrap();
                (
                    capture.start(),
                    capture.end(),
                    captures[1].to_string(),
                    captures[2].to_string(),
                    captures.get(3).map(|group| group.as_str().to_string()),
                    captures.get(4).map(|group| group.as_str().to_string()),
                )
            }))
            .map(|(begin, end, org, name, version, content)| ParseItem {
                begin,
                end,
                node: Node::SoftwareSourceCode(SoftwareSourceCode {
                    code_repository: Some(Box::new(format!("https://gitlab.com/{}/{}", org, name))),
                    publisher: Some(Box::new(CreativeWorkPublisher::Organization(
                        Organization {
                            name: Some(Box::new(org)),
                            ..Default::default()
                        },
                    ))),
                    name: Some(Box::new(name)),
                    version: version.map(|version| Box::new(CreativeWorkVersion::String(version))),
                    content: content.map(|path| Box::new(CreativeWorkContent::String(path))),
                    ..Default::default()
                }),
            })
            .collect()
    }

    async fn enrich(node: Node, options: Option<EnrichOptions>) -> Result<Node> {
        let ssc = match &node {
            Node::SoftwareSourceCode(ssc) => ssc,
            _ => return Ok(node),
        };
        let project_id = match Self::project_id(ssc) {
            Some(project_id) => project_id,
            None => return Ok(node),
        };

        let options = options.unwrap_or_default();

        let client = GitlabClient::new(options.token)?;
        let project: Project = client.get(&format!("projects/{}", project_id)).await?;

        let description = match !project.description.is_empty() {
            true => Some(Box::new(ThingDescription::String(project.description))),
            false => None,
        };

        let date_created = Some(Box::new(Date::from(project.created_at)));

        let ssc = SoftwareSourceCode {
            description,
            date_created,
            ..ssc.clone()
        };

        Ok(Node::SoftwareSourceCode(ssc))
    }

    async fn import(node: &Node, dest: &Path, options: Option<ImportOptions>) -> Result<bool> {
        let ssc = match node {
            Node::SoftwareSourceCode(ssc) => ssc,
            _ => return Ok(false),
        };
        let project_id = match Self::project_id(ssc) {
            Some(project_id) => project_id,
            None => return Ok(false),
        };

        let options = options.unwrap_or_default();
        let client = GitlabClient::new(options.token)?;

        let ref_ = Self::version(ssc).unwrap_or("HEAD").to_string();
        let path = Self::path(ssc).unwrap_or_default();
        let path_id = Self::path_id(ssc).unwrap_or_default();

        // Attempt to get the path as a single file
        let repo_file = match client
            .get::<RepositoryFile>(&format!(
                "projects/{id}/repository/files/{path}?ref={ref_}",
                id = project_id,
                path = path_id,
                ref_ = ref_
            ))
            .await
        {
            Ok(repo_file) => Some(repo_file),
            Err(error) => {
                if !error.to_string().contains("404 Not Found") {
                    bail!("While fetching file from Gitlab: {}", error)
                } else {
                    // Not a file, so will attempt to extract as folder below
                    None
                }
            }
        };

        if let Some(repo_file) = repo_file {
            // Content is a single file with content so write to destination
            let name = PathBuf::from(&repo_file.file_name);
            if let Some(dest_ext) = dest.extension() {
                let dest_ext = dest_ext.to_string_lossy().to_string();
                let source_ext = name.extension().map_or_else(
                    || repo_file.file_name.to_string(),
                    |os_str| os_str.to_string_lossy().to_string(),
                );
                if source_ext == dest_ext {
                    // Destination format is same as content so just write it
                    repo_file.write(dest)?
                } else {
                    // Destination has a different format so convert it first
                    codecs::str_to_path(&repo_file.content()?, &source_ext, dest, None).await?;
                }
            } else {
                // Destination has no extension so treat it as a directory and write the file into it
                repo_file.write(&dest.join(&repo_file.file_name))?;
            }
        } else {
            // Content is a directory so fetch the whole repo as a tarball and extract the directory
            // (getting the whole rpo as a tarball is more efficient than making lots of small requests
            // for each file's content - for most repos)
            tracing::info!("Downloading repository tarball");
            let archive = client
                .download_temp(&format!(
                    "projects/{id}/repository/archive?sha={sha}&path={path}",
                    id = project_id,
                    sha = ref_,
                    path = path_id
                ))
                .await?;

            // Extract the part of the archive we want
            create_dir_all(dest)?;
            extract_tar(
                "gz",
                archive.path(),
                dest,
                1,
                if path.is_empty() { None } else { Some(path) },
            )?;
        }

        Ok(true)
    }

    async fn sync(node: &Node, local: &Path, options: Option<SyncOptions>) -> Result<bool> {
        let ssc = match node {
            Node::SoftwareSourceCode(ssc) => ssc,
            _ => return Ok(false),
        };
        let project_id = match Self::project_id(ssc) {
            Some(project_id) => project_id,
            None => return Ok(false),
        };

        let version = Self::version(ssc);
        let path = Self::path(ssc);
        let path_id = Self::path_id(ssc).unwrap_or_default();

        let options = options.unwrap_or_default();
        let client = GitlabClient::new(options.token)?;

        // Get a local URL
        let url = options.url.unwrap_or_default();

        // Generate a token for validating event payloads
        let token = key_utils::generate().to_string();

        // Create the webhook
        let hook: Hook = client
            .post(
                &format!("projects/{id}/hooks", id = project_id),
                json!({
                    "url": url,
                    "token": token,
                    "push_events": true
                }),
            )
            .await?;
        tracing::info!("Created Gitlab webhook `{}`", hook.url);

        // Listen for webhook events
        let response_headers = Headers(vec![(
            "Server",
            format!(
                "Stencila-Gitlab-Provider/{} ({})",
                env!("CARGO_PKG_VERSION"),
                env::consts::OS
            ),
        )]);
        let local_clone = local.to_path_buf();
        let client_clone = client.clone();
        let project_clone = project_id.clone();
        let ref_ = version
            .map(|version| ["refs/heads/", version].concat())
            .unwrap_or_else(|| "refs/heads/main".to_string());
        let path_clone = path.map(|path| path.to_string()).unwrap_or_default();
        let router = Router::new().route(
            "/",
            routing::post(
                // Note that the order of extractors is important and some may not be able to be mixed.
                // See https://docs.rs/axum/latest/axum/extract/index.html#applying-multiple-extractors
                move |Json(event): Json<HookEvent>, request_headers: HeaderMap| async move {
                    match webhook_event(
                        request_headers,
                        event,
                        &local_clone,
                        &client_clone,
                        &token,
                        &project_clone,
                        &ref_,
                        &path_clone,
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
                },
            ),
        );
        serve_gracefully([0, 0, 0, 0], WATCH_SERVER_PORT, router).await?;

        // Delete the webhook
        match client
            .delete(&format!(
                "projects/{project_id}/hooks/{hook_id}",
                project_id = project_id,
                hook_id = hook.id
            ))
            .await
        {
            Ok(..) => {
                tracing::info!("Deleted Gitlab webhook `{}`", hook.id);
            }
            Err(error) => {
                tracing::warn!("While deleting Gitlab webhook: {}", error);
            }
        }

        Ok(true)
    }
}

/// A project
///
/// See https://docs.gitlab.com/ee/api/projects.html#get-single-project
#[derive(Debug, Deserialize)]
#[serde(crate = "http_utils::serde")]
struct Project {
    description: String,
    created_at: String,
}

/// A file in a repository
///
/// See https://docs.gitlab.com/ee/api/repository_files.html#get-file-from-repository
#[derive(Debug, Deserialize)]
#[serde(crate = "http_utils::serde")]
struct RepositoryFile {
    file_name: String,
    content: String,
}

impl RepositoryFile {
    /// Decode Base64 content to string
    fn content(&self) -> Result<String> {
        Ok(base64::decode(&self.content)
            .map(|slice| String::from_utf8_lossy(&slice).to_string())?)
    }

    /// Write content to disk
    fn write(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            create_dir_all(parent)?
        }
        let mut file = File::create(path)?;
        file.write_all(&base64::decode(&self.content)?)?;
        Ok(())
    }
}

/// A webhook
///
/// See https://docs.gitlab.com/ee/api/projects.html#get-project-hook
#[derive(Debug, Deserialize)]
#[serde(crate = "http_utils::serde")]
struct Hook {
    id: u64,
    url: String,
}

/// A webhook push event
///
/// See https://docs.gitlab.com/ee/user/project/integrations/webhook_events.html#push-events
#[derive(Debug, Deserialize)]
#[serde(crate = "http_utils::serde")]
struct HookEvent {
    #[serde(rename = "ref")]
    ref_: String,
    commits: Vec<HookEventCommit>,
}

/// A commit within a webhook push event
#[derive(Debug, Deserialize)]
#[serde(crate = "http_utils::serde")]
struct HookEventCommit {
    added: Vec<String>,
    modified: Vec<String>,
    removed: Vec<String>,
}

/// Handle Gitlab webhook events
///
/// Validates payloads using secret token.
/// See https://docs.gitlab.com/ee/user/project/integrations/webhooks.html#validate-payloads-by-using-a-secret-token.
///
/// For debugging purposes this function both logs events and returns meaningful status codes
/// and messages for recording in the "Recent events" log on Gitlab.
/// See https://docs.gitlab.com/ee/user/project/integrations/webhooks.html#troubleshoot-webhooks
#[allow(clippy::too_many_arguments)]
async fn webhook_event(
    headers: HeaderMap,
    event: HookEvent,
    local: &Path,
    client: &GitlabClient,
    token: &str,
    project_id: &str,
    ref_: &str,
    path: &str,
) -> Result<(StatusCode, String)> {
    // Reject events with a nonexistent or invalid token
    match headers.get("X-Gitlab-Token") {
        Some(value) => {
            if value != token {
                let msg = "Rejected webhook event with an invalid token";
                tracing::warn!("{}", msg);
                return Ok((StatusCode::BAD_REQUEST, msg.into()));
            }
        }
        None => {
            let msg = "Rejected webhook event without token";
            tracing::warn!("{}", msg);
            return Ok((StatusCode::BAD_REQUEST, msg.into()));
        }
    };

    // Ignore events not associated with the ref being watched
    if event.ref_.is_empty() {
        // Not a push event
        let msg = "Ignoring non-push webhook event";
        tracing::trace!("{}", msg);
        return Ok((StatusCode::ACCEPTED, msg.into()));
    }
    if !(event.ref_ == ref_ || (event.ref_ == "refs/heads/master" && ref_ == "refs/heads/main")) {
        let msg = format!(
            "Ignoring webhook event for a different ref `{} != {}`",
            event.ref_, ref_
        );
        tracing::trace!("{}", msg);
        return Ok((StatusCode::ACCEPTED, msg));
    }

    // Iterate over the commits and synchronize each file
    for commit in event.commits {
        const ADDED: &str = "added";
        const MODIFIED: &str = "modified";
        const REMOVED: &str = "removed";
        for action in [ADDED, MODIFIED, REMOVED] {
            let paths = match action {
                ADDED => &commit.added,
                MODIFIED => &commit.modified,
                REMOVED => &commit.removed,
                _ => unreachable!(),
            };
            for event_path in paths {
                let local_path = match PathBuf::from(event_path).strip_prefix(path) {
                    // Only join stripped path if it has content. This avoids a trailing slash
                    // when the local path is a file
                    Ok(path) => match path == PathBuf::from("") {
                        true => local.to_path_buf(),
                        false => local.join(path),
                    },
                    Err(..) => {
                        tracing::info!(
                            "Ignored webhook event with excluded path: `{}` is not in `{}`",
                            event_path,
                            path
                        );
                        continue;
                    }
                };

                if action == ADDED || action == MODIFIED {
                    // Fetch the content of the file and write to disk
                    tracing::info!(
                        "Fetching content of `{}` to write to `{}`",
                        event_path,
                        local_path.display()
                    );
                    let repo_file = client
                        .get::<RepositoryFile>(&format!(
                            "projects/{id}/repository/files/{path}?ref={ref_}",
                            id = project_id,
                            path = urlencoding::encode(event_path),
                            ref_ = event.ref_
                        ))
                        .await?;
                    repo_file.write(&local_path)?;
                } else {
                    // Remove the file, if it exists
                    if local_path.exists() {
                        remove_file(local_path)?;
                    } else {
                        tracing::warn!(
                            "Ignored webhook event to remove non-existent file `{}`",
                            local_path.display()
                        );
                    }
                }
            }
        }
    }

    let msg = "Webhook event handled";
    tracing::trace!("{}", msg);
    Ok((StatusCode::OK, msg.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::assert_json_is;

    #[test]
    fn parse() {
        // No path or version
        for string in [
            "gitlab:owner/name",
            "http://gitlab.com/owner/name",
            "https://gitlab.com/owner/name",
            "https://gitlab.com/owner/name/",
            "https://gitlab.com/owner/name/-/tree",
            "https://gitlab.com/owner/name/-/blob/",
        ] {
            assert_json_is!(
                GitlabProvider::parse(string)[0].node,
                {
                    "type": "SoftwareSourceCode",
                    "codeRepository": "https://gitlab.com/owner/name",
                    "publisher": {
                        "type": "Organization",
                        "name": "owner"
                    },
                    "name": "name",
                }
            );
        }

        // Version, no path
        for string in [
            "gitlab:owner/name@version",
            "https://gitlab.com/owner/name/-/tree/version/",
        ] {
            assert_json_is!(
                GitlabProvider::parse(string)[0].node,
                {
                    "type": "SoftwareSourceCode",
                    "codeRepository": "https://gitlab.com/owner/name",
                    "publisher": {
                        "type": "Organization",
                        "name": "owner"
                    },
                    "name": "name",
                    "version": "version"
                }
            );
        }

        // Folder path and version
        for string in [
            "gitlab:owner/name/sub/folder@version",
            "https://gitlab.com/owner/name/-/tree/version/sub/folder",
        ] {
            assert_json_is!(
                GitlabProvider::parse(string)[0].node,
                {
                    "type": "SoftwareSourceCode",
                    "codeRepository": "https://gitlab.com/owner/name",
                    "publisher": {
                        "type": "Organization",
                        "name": "owner"
                    },
                    "name": "name",
                    "version": "version",
                    "content": "sub/folder"
                }
            );
        }

        // File path and version
        for string in [
            "gitlab:owner/name/sub/folder/file.ext@version",
            "https://gitlab.com/owner/name/-/blob/version/sub/folder/file.ext",
        ] {
            assert_json_is!(
                GitlabProvider::parse(string)[0].node,
                {
                    "type": "SoftwareSourceCode",
                    "codeRepository": "https://gitlab.com/owner/name",
                    "publisher": {
                        "type": "Organization",
                        "name": "owner"
                    },
                    "name": "name",
                    "version": "version",
                    "content": "sub/folder/file.ext"
                }
            );
        }

        // File path, no version (only for short identifier)
        for string in ["gitlab:owner/name/sub/folder/file.ext"] {
            assert_json_is!(
                GitlabProvider::parse(string)[0].node,
                {
                    "type": "SoftwareSourceCode",
                    "codeRepository": "https://gitlab.com/owner/name",
                    "publisher": {
                        "type": "Organization",
                        "name": "owner"
                    },
                    "name": "name",
                    "content": "sub/folder/file.ext"
                }
            );
        }
    }
}
