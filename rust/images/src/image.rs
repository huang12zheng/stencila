use std::{
    collections::HashMap,
    env,
    path::{Path, PathBuf},
};

use oci_spec::image::{
    Descriptor, History, HistoryBuilder, ImageConfiguration, ImageConfigurationBuilder,
    ImageIndexBuilder, ImageManifestBuilder, MediaType, RootFsBuilder, SCHEMA_VERSION,
};

use common::{
    chrono::Utc,
    eyre::{bail, eyre, Result},
    glob,
    serde::Serialize,
    serde_json, tokio, tracing,
};
use hash_utils::str_sha256_hex;

use crate::{
    blob_writer::BlobWriter,
    distribution::Client,
    image_reference::ImageReference,
    media_types::ToDockerV2S2,
    snapshot::Snapshot,
    storage::{image_path, image_path_safe, write_oci_layout_file, IMAGES_MAP},
    utils::unique_string,
};

/// A container image
///
/// This is serializable mainly so that it can be inspected as JSON or YAML output from a CLI command.
#[derive(Debug, Serialize)]
#[serde(crate = "common::serde")]
pub struct Image {
    /// The working directory to build an image for
    ///
    /// Buildpacks will build layers based on the source code within this directory. Usually
    /// the home directory of a project. Defaults to the current directory.
    working_dir: Option<PathBuf>,

    /// The image reference for this image
    reference: ImageReference,

    /// The image reference for the base image from which this image is derived
    ///
    /// Equivalent to the `FROM` directive of a Dockerfile.
    base: ImageReference,

    /// The directory that contains the buildpack layers
    ///
    /// Defaults to `/layers` or `<working_dir>/.stencila/layers` (in order of priority).
    layers_dir: PathBuf,

    /// Whether snapshots should be diffed or replicated
    layer_diffs: bool,

    /// The format used when writing layers
    layer_format: MediaType,

    /// The snapshots for each layer directory, used to generated [`ChangeSet`]s and image layers
    #[serde(skip)]
    layer_snapshots: Vec<Snapshot>,

    /// The format for the image manifest
    ///
    /// Defaults to `application/vnd.oci.image.manifest.v1+json`. However, for some registries it
    /// may be necessary to use `application/vnd.docker.distribution.manifest.v2+json` (which has
    /// the same underlying schema).
    manifest_format: String,
}

impl Image {
    /// Create a new image
    pub fn new(
        working_dir: Option<&Path>,
        ref_: Option<&str>,
        base: Option<&str>,
        layers_dir: Option<&Path>,
        layer_diffs: Option<bool>,
        layer_format: Option<&str>,
        manifest_format: Option<&str>,
    ) -> Result<Self> {
        let working_dir = working_dir.map(PathBuf::from);

        let ref_ = match ref_ {
            Some(reference) => reference.parse::<ImageReference>()?,
            None => {
                let registry = "localhost".to_string();

                let name = working_dir
                    .as_ref()
                    .and_then(|dir| dir.file_name())
                    .map(|name| name.to_string_lossy().to_string())
                    .unwrap_or_else(|| "unnamed".to_string());
                let hash = working_dir
                    .as_ref()
                    .map(|dir| str_sha256_hex(&dir.to_string_lossy().to_string()))
                    .unwrap_or_else(unique_string);
                let repository = [&name, "-", &hash[..12]].concat();

                ImageReference {
                    registry,
                    repository,
                    ..Default::default()
                }
            }
        };

        let base = base
            .map(String::from)
            .or_else(|| std::env::var("STENCILA_IMAGE_REF").ok())
            .unwrap_or_else(|| "ubuntu".to_string())
            .parse()?;

        let layers_dir = layers_dir
            .map(|path| path.to_path_buf())
            .unwrap_or_else(|| {
                let layers_top = PathBuf::from("/layers");
                if layers_top.exists() {
                    layers_top
                } else if let Some(working_dir) = working_dir.as_ref() {
                    let dir = working_dir.join(".stencila").join("layers");
                    std::fs::create_dir_all(&dir).expect("Unable to create layers dir");
                    dir
                } else {
                    std::env::temp_dir().join(["stencila-", &unique_string()].concat())
                }
            });

        // Before creating snapshots do a "prebuild" so that the directories
        // that may need to be snapshotted are present and picked up in `layers_dir.read_dir()` call below.
        buildpacks::PACKS.prebuild_all(&layers_dir)?;

        let mut layer_snapshots = Vec::new();
        if let Some(working_dir) = working_dir.as_ref() {
            layer_snapshots.push(Snapshot::new(working_dir.clone(), "/workspace"));
        }
        for subdir in layers_dir.read_dir()?.flatten().filter_map(|entry| {
            if entry.path().is_dir() {
                Some((entry.path(), entry.file_name()))
            } else {
                None
            }
        }) {
            layer_snapshots.push(Snapshot::new(
                &subdir.0,
                PathBuf::from("/layers").join(subdir.1),
            ));
        }

        let layer_diffs = layer_diffs.unwrap_or(true);

        let layer_format = match layer_format {
            None | Some("tar+gzip") | Some("tgz") => MediaType::ImageLayerGzip,
            Some("tar+zstd") | Some("tzs") => MediaType::ImageLayerZstd,
            Some("tar") => MediaType::ImageLayer,
            _ => bail!("Unknown layer format"),
        };

        let manifest_format = match manifest_format {
            None | Some("oci") => MediaType::ImageManifest.to_string(),
            Some("v2s2") => MediaType::ImageManifest.to_docker_v2s2()?.to_string(),
            _ => bail!("Unknown manifest format"),
        };

        Ok(Self {
            working_dir,
            reference: ref_,
            base,
            layers_dir,
            layer_snapshots,
            layer_diffs,
            layer_format,
            manifest_format,
        })
    }

    /// Get the [`ImageReference`] of the image
    pub fn reference(&self) -> &ImageReference {
        &self.reference
    }

    /// Get the [`ImageReference`] of the image's base
    pub fn base(&self) -> &ImageReference {
        &self.base
    }

    /// Fetches the manifest and configuration of the base image
    ///
    /// Used when writing the image because the DiffIDs (from the config) and the layers (from the
    /// manifest) and required for the config and manifest of this image.
    async fn get_base(&self) -> Result<(String, ImageConfiguration, Vec<Descriptor>)> {
        let client = Client::new(&self.base.registry, &self.base.repository, None).await?;
        let (manifest, manifest_descriptor) = client
            .pull_manifest(&self.base.digest_or_tag_or_latest(), None)
            .await?;

        let config = client.pull_config(&manifest, None).await?;
        let layers = manifest.layers().clone();

        Ok((manifest_descriptor.digest().to_string(), config, layers))
    }

    /// Write the image layer blobs and returns vectors of DiffIDs and layer descriptors
    async fn write_layers(
        &self,
        base_config: &ImageConfiguration,
        base_layers: Vec<Descriptor>,
        layout_dir: &Path,
    ) -> Result<(Vec<Descriptor>, Vec<String>, Vec<History>)> {
        let mut layers = base_layers;
        let mut diff_ids = base_config.rootfs().diff_ids().clone();
        let mut histories = base_config.history().clone();

        let client = Client::new(&self.base.registry, &self.base.repository, None).await?;
        for layer in &layers {
            client.pull_blob_via(layer, None).await?
        }

        for snapshot in &self.layer_snapshots {
            let (diff_id, layer) =
                snapshot.write_layer(layout_dir, self.layer_diffs, &self.layer_format)?;

            if diff_id == "<empty>" {
                continue;
            }

            diff_ids.push(diff_id);
            layers.push(layer);

            let history = HistoryBuilder::default()
                .created(Utc::now().to_rfc3339())
                .created_by(format!(
                    "stencila {}",
                    env::args().skip(1).collect::<Vec<String>>().join(" ")
                ))
                .comment(format!("Layer for directory {}", snapshot.source_dir))
                .build()?;
            histories.push(history)
        }

        Ok((layers, diff_ids, histories))
    }

    /// Write the image config blob
    ///
    /// Implements the [OCI Image Configuration Specification](https://github.com/opencontainers/image-spec/blob/main/config.md).
    fn write_config(
        &self,
        base_config: &ImageConfiguration,
        diff_ids: Vec<String>,
        history: Vec<History>,
        layout_dir: &Path,
    ) -> Result<Descriptor> {
        // Start with the config of the base image and override as necessary
        let mut config = base_config.config().clone().unwrap_or_default();

        // Working directory is represented in image as /workspace. Override it
        config.set_working_dir(Some("/workspace".to_string()));

        let layers_dir = self
            .layers_dir
            .to_str()
            .ok_or_else(|| eyre!("Layers dir is none"))?;

        // Get the environment variables in the base images
        let mut env: HashMap<String, String> = config
            .env()
            .clone()
            .unwrap_or_default()
            .iter()
            .map(|name_value| {
                let mut parts = name_value.splitn(2, '=');
                (
                    parts.next().unwrap_or_default().to_owned(),
                    parts.next().unwrap_or_default().to_owned(),
                )
            })
            .collect();

        // Update the env vars with those that are expected to be provided by the `launcher` lifecycle
        // See https://github.com/buildpacks/spec/blob/main/buildpack.md#provided-by-the-lifecycle
        let layer_dirs = glob::glob(&[layers_dir, "/*/*/"].concat())?.flatten();
        for layer_dir in layer_dirs {
            let path = [
                layer_dir.join("bin").to_string_lossy().to_string(),
                ":".to_string(),
                env.get("PATH").cloned().unwrap_or_default(),
            ]
            .concat();
            env.insert("PATH".to_string(), path);

            let lid_library_path = [
                layer_dir.join("lib").to_string_lossy().to_string(),
                ":".to_string(),
                env.get("LD_LIBRARY_PATH").cloned().unwrap_or_default(),
            ]
            .concat();
            env.insert("LD_LIBRARY_PATH".to_string(), lid_library_path);
        }

        // Update the env vars with those provided by buildpacks
        // See https://github.com/buildpacks/spec/blob/main/buildpack.md#provided-by-the-buildpacks
        let var_files = glob::glob(&[layers_dir, "/*/*/env/*"].concat())?.flatten();
        for var_file in var_files {
            let action = match var_file.extension() {
                Some(ext) => ext.to_string_lossy().to_string(),
                None => continue,
            };
            let name = match var_file.file_stem() {
                Some(name) => name.to_string_lossy().to_string(),
                None => continue,
            };
            let mut value = match env.get(&name) {
                Some(value) => value.clone(),
                None => String::new(),
            };
            let new_value = std::fs::read_to_string(&var_file)?;

            // Apply modification action
            //
            // Because the base image may have been built with Stencila buildpacks, for
            // prepend and append the value is only added if it is not present (this avoid
            // having env vars such as PATH that grow very long).
            //
            // Because prepend and append are almost always associated with paths, and need
            // to have separator, these are added if necessary.
            match action.as_str() {
                "default" => {
                    if value.is_empty() {
                        value = new_value;
                    }
                }
                "prepend" => {
                    if !value.contains(&new_value) {
                        let sep = if new_value.ends_with(':')
                            || value.is_empty()
                            || value.starts_with(':')
                        {
                            ""
                        } else {
                            ":"
                        };
                        value = [new_value, sep.to_string(), value].concat()
                    }
                }
                "append" => {
                    if !value.contains(&new_value) {
                        let sep = if new_value.starts_with(':')
                            || value.is_empty()
                            || value.ends_with(':')
                        {
                            ""
                        } else {
                            ":"
                        };
                        value = [value, sep.to_string(), new_value].concat()
                    }
                }
                "override" => {
                    value = new_value;
                }
                _ => tracing::warn!("ignoring env var file {}", var_file.display()),
            }

            env.insert(name, value);
        }

        // Add an env var for the ref of the image (used as the default `--from` image when building another image from this)
        env.insert("STENCILA_IMAGE_REF".to_string(), self.reference.to_string());

        let env: Vec<String> = env
            .iter()
            .map(|(name, value)| [name, "=", value].concat())
            .collect();
        config.set_env(Some(env));

        // Extend labels, including with the contents of an y `.image-labels` file in working dir
        let mut labels = config.labels().clone().unwrap_or_default();
        labels.insert(
            "io.stencila.version".to_string(),
            env!("CARGO_PKG_VERSION").to_string(),
        );
        if let Some(content) = self
            .working_dir
            .as_ref()
            .and_then(|dir| std::fs::read_to_string(dir.join(".image-labels")).ok())
        {
            for line in content.lines() {
                if let Some((name, value)) = line.split_once(' ') {
                    labels.insert(name.into(), value.into());
                }
            }
        }
        config.set_labels(Some(labels));

        // Rootfs DiffIDs calculated above
        let rootfs = RootFsBuilder::default().diff_ids(diff_ids).build()?;

        let configuration = ImageConfigurationBuilder::default()
            .created(Utc::now().to_rfc3339())
            .os(env::consts::OS)
            // Not that arch should be one of the values listed at https://go.dev/doc/install/source#environment
            // and that `env::consts::ARCH` does not necessarily return that
            .architecture("amd64")
            .config(config)
            .rootfs(rootfs)
            .history(history)
            .build()?;

        BlobWriter::write_json(
            &configuration,
            MediaType::ImageConfig,
            None,
            Some(layout_dir),
        )
    }

    /// Write the image manifest blob
    ///
    /// Implements the [OCI Image Manifest Specification](https://github.com/opencontainers/image-spec/blob/main/manifest.md).
    /// Given that the manifest requires the descriptors for config and layers also calls `write_config` and `write_layers`.
    async fn write_manifest(&self, layout_dir: &Path) -> Result<(String, Descriptor, Descriptor)> {
        let (base_digest, base_config, base_layers) = self.get_base().await?;

        let (layers, diff_ids, history) = self
            .write_layers(&base_config, base_layers, layout_dir)
            .await?;

        let config_descriptor = self.write_config(&base_config, diff_ids, history, layout_dir)?;

        let manifest = ImageManifestBuilder::default()
            .schema_version(SCHEMA_VERSION)
            .media_type(self.manifest_format.as_str())
            .config(config_descriptor.clone())
            .layers(layers)
            .build()?;

        let manifest_descriptor =
            BlobWriter::write_json(&manifest, MediaType::ImageManifest, None, Some(layout_dir))?;

        Ok((base_digest, config_descriptor, manifest_descriptor))
    }

    /// Write the image `index.json`
    ///
    /// Implements the [OCI Image Index Specification](https://github.com/opencontainers/image-spec/blob/main/image-index.md).
    /// Updates both `self.ref_.digest` and `self.base.digest`.
    async fn write_index(&mut self, layout_dir: &Path) -> Result<Descriptor> {
        use tokio::fs;

        let (base_digest, config_descriptor, manifest_descriptor) =
            self.write_manifest(layout_dir).await?;

        self.base.digest = Some(base_digest.clone());
        self.reference.digest = Some(manifest_descriptor.digest().to_string());

        let annotations: HashMap<String, String> = [
            // Where appropriate use pre defined annotations
            // https://github.com/opencontainers/image-spec/blob/main/annotations.md#pre-defined-annotation-keys
            (
                "org.opencontainers.image.ref.name".to_string(),
                self.reference.to_string_tag_or_latest(),
            ),
            (
                "org.opencontainers.image.created".to_string(),
                Utc::now().to_rfc3339(),
            ),
            (
                "org.opencontainers.image.base.name".to_string(),
                self.base.to_string_tag_or_latest(),
            ),
            (
                "org.opencontainers.image.base.digest".to_string(),
                base_digest,
            ),
        ]
        .into();

        let index = ImageIndexBuilder::default()
            .schema_version(SCHEMA_VERSION)
            .media_type(MediaType::ImageIndex)
            .manifests([manifest_descriptor])
            .annotations(annotations)
            .build()?;
        fs::write(
            layout_dir.join("index.json"),
            serde_json::to_string_pretty(&index)?,
        )
        .await?;

        Ok(config_descriptor)
    }

    pub async fn build(&self) -> Result<()> {
        if let Some(working_dir) = &self.working_dir {
            // Because buildpacks will change directories into the working dir. It is safest to use absolute paths here.
            let working_dir = working_dir.canonicalize()?;
            let layers_dir = self.layers_dir.canonicalize()?;

            buildpacks::PACKS.build_all(Some(&working_dir), Some(&layers_dir), None)?;
        }

        Ok(())
    }

    /// Write the image to `layout_dir`
    ///
    /// Implements the [OCI Image Layout Specification](https://github.com/opencontainers/image-spec/blob/main/image-layout.md).
    ///
    /// Note that the `blobs/sha256` subdirectory may not have blobs for the base image (these
    /// are only pulled into that directory if necessary i.e. if the registry does not yet have them).
    pub async fn write(&mut self) -> Result<()> {
        use tokio::fs;

        // Create a temporary OCI layout directory
        let layout_dir = image_path(&format!("temp:{}", &unique_string()));

        // Write image into that directory
        let config_descriptor = self.write_index(&layout_dir).await?;
        write_oci_layout_file(&layout_dir)?;
        let config_digest = config_descriptor.digest();

        // Now we know the id of the image, rename the dir
        let image_dir = image_path_safe(config_digest)?;
        fs::rename(&layout_dir, image_dir).await?;

        // Add an entry in the images map
        let mut images = IMAGES_MAP.write().await;
        images.insert(&self.reference.to_string_tag_or_latest(), config_digest)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test_utils::common::tempfile::tempdir;

    use super::*;

    /// Test that when an image is written to a directory that they directory conforms to
    /// the OCI Image Layout spec
    #[tokio::test]
    async fn image_write() -> Result<()> {
        let working_dir = tempdir()?;
        let mut image = Image::new(
            Some(working_dir.path()),
            None,
            Some("ubuntu"),
            None,
            None,
            None,
            None,
        )?;

        image.write().await?;

        //assert!(image.layout_dir.join("oci-layout").is_file());
        //assert!(image.layout_dir.join("index.json").is_file());
        //assert!(image.layout_dir.join("blobs").join("sha256").is_dir());

        Ok(())
    }
}