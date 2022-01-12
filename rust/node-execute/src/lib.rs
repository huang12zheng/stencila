// Re-exports
pub use kernels::{KernelSelector, KernelSpace, TaskResult};

mod executable;
pub use executable::*;

mod compile;
pub use compile::*;

mod execute;
pub use execute::*;

#[cfg(test)]
mod tests {
    use super::*;
    use codec::CodecTrait;
    use codec_md::MdCodec;
    use eyre::Result;
    use graph::{PlanOptions, PlanOrdering};
    use graph_triples::ResourceInfo;
    use kernels::{Kernel, KernelType};
    use node_address::Slot;
    use node_patch::{Operation, Patch};
    use test_snaps::{
        fixtures,
        insta::{self, assert_json_snapshot},
        snapshot_set_suffix,
    };
    use tokio::sync::mpsc;

    /// Higher level tests of the top level functions in this crate
    #[tokio::test]
    async fn md_articles() -> Result<()> {
        let fixtures = fixtures();

        // So that test results are not dependant upon the the machine the test is run on or how
        // the test is compiled only use built-in kernels
        let kernels: Vec<Kernel> = kernels::available()
            .await?
            .into_iter()
            .filter(|kernel| matches!(kernel.r#type, KernelType::Builtin))
            .collect();

        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.bind_to_thread();

        for name in ["code.md", "code-relations.md"] {
            let path = fixtures.join("articles").join(name);

            // Load the article
            let mut root = MdCodec::from_path(&path, None).await?;

            // Strip the fixtures path so it does not differ between machines
            let path = path.strip_prefix(&fixtures)?;
            let project = path.parent().unwrap();

            // Compile the article and snapshot the result
            let (patch_sender, mut patch_receiver) = mpsc::unbounded_channel();
            tokio::spawn(async move {
                while let Some(_patch) = patch_receiver.recv().await {
                    // Ignore for this test
                }
            });
            let (addresses, graph) = compile(path, project, &mut root, &patch_sender)?;
            snapshot_set_suffix(&[name, "-compile"].concat(), || {
                assert_json_snapshot!((&addresses, &graph))
            });

            // Generate various execution plans for the article using alternative options
            // and snapshot them all. Always specify `max_concurrency` to avoid differences
            // due to machine (number of CPUs)
            for (suffix, options) in [
                (
                    "appearance",
                    PlanOptions {
                        ordering: PlanOrdering::Appearance,
                        max_concurrency: 10,
                    },
                ),
                (
                    "appearance-concurrency-0",
                    PlanOptions {
                        ordering: PlanOrdering::Appearance,
                        max_concurrency: 0,
                    },
                ),
                (
                    "topological",
                    PlanOptions {
                        ordering: PlanOrdering::Topological,
                        max_concurrency: 10,
                    },
                ),
                (
                    "topological-concurrency-0",
                    PlanOptions {
                        ordering: PlanOrdering::Topological,
                        max_concurrency: 0,
                    },
                ),
            ] {
                let plan = graph
                    .plan(None, Some(kernels.clone()), Some(options))
                    .await?;
                snapshot_set_suffix(&[name, "-", suffix].concat(), || {
                    assert_json_snapshot!(&plan)
                });
            }

            // Execute the article (with topological execution plan) and snapshot the resultant patches
            let plan = graph
                .plan(
                    None,
                    Some(kernels.clone()),
                    Some(PlanOptions {
                        ordering: PlanOrdering::Topological,
                        max_concurrency: 10,
                    }),
                )
                .await?;

            let (resource_info_sender, mut resource_info_receiver) =
                mpsc::channel::<ResourceInfo>(1);
            tokio::spawn(async move {
                while let Some(_resource_info) = resource_info_receiver.recv().await {
                    // Ignore for this test
                }
            });

            let (patch_sender, mut patch_receiver) = mpsc::unbounded_channel::<Patch>();
            let patches = tokio::spawn(async move {
                let mut patches = Vec::new();
                while let Some(mut patch) = patch_receiver.recv().await {
                    // Redact execute time and duration from patch because they will
                    // change across test runs
                    for op in patch.ops.iter_mut() {
                        if let Operation::Add { address, value, .. } = op {
                            if let Some(Slot::Name(name)) = address.back() {
                                if name == "execute_ended" || name == "execute_duration" {
                                    *value = Box::new("<redacted>".to_string());
                                }
                            }
                        }
                    }
                    patches.push(patch);
                }
                patches
            });

            execute(
                &plan,
                &mut root,
                &addresses,
                resource_info_sender,
                patch_sender,
                None,
            )
            .await?;

            let patches = patches.await?;
            snapshot_set_suffix(&[name, "-patches"].concat(), || {
                assert_json_snapshot!(&patches);
            });
        }

        Ok(())
    }
}
