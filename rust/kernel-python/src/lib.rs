use kernel_micro::{include_file, MicroKernel};

/// A microkernel for Python
pub fn new() -> MicroKernel {
    MicroKernel::new(
        "python-micro",
        &["python"],
        true,
        true,
        cfg!(any(target_os = "linux", target_os = "macos")),
        ("python3", "*"),
        &["{{script}}"],
        include_file!("python_kernel.py"),
        &[include_file!("python_codec.py")],
        "{{name}} = decode_value(\"{{json}}\")",
        "{{name}}",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use kernel::{
        eyre::{bail, Result},
        stencila_schema::Node,
        KernelTrait,
    };
    use kernel_micro::MicroKernelInterrupter;
    use test_utils::{assert_json_eq, assert_json_is, skip_ci_os};

    async fn skip_or_kernel() -> Result<MicroKernel> {
        if skip_ci_os("windows", "Failing on Windows CIs") {
            bail!("Skipping")
        }

        let mut kernel = new();
        if !kernel.available().await {
            eprintln!("Python not available on this machine");
            bail!("Skipping")
        } else {
            kernel.start().await?;
        }

        Ok(kernel)
    }

    /// Tests of basic functionality
    /// This test is replicated in all the microkernels.
    /// Other test should be written for language specific quirks and regressions.
    #[tokio::test]
    async fn basics() -> Result<()> {
        let mut kernel = match skip_or_kernel().await {
            Ok(kernel) => kernel,
            Err(..) => return Ok(()),
        };

        // The execution context should start off empty
        let (outputs, messages) = kernel.exec("dir()").await?;
        assert_json_is!(messages, []);
        assert_json_is!(outputs, [[]]);

        // Assign a variable and output it
        let (outputs, messages) = kernel.exec("a = 2\na").await?;
        assert_json_is!(messages, []);
        assert_json_is!(outputs, [2]);

        // The execution context should now have the var
        let (outputs, messages) = kernel.exec("dir()").await?;
        assert_json_is!(messages, []);
        assert_json_is!(outputs, [["a"]]);

        // Print the variable twice and then output it
        let (outputs, messages) = kernel.exec("print(a)\nprint(a)\na").await?;
        assert_json_is!(messages, []);
        assert_json_is!(outputs, [2, 2, 2]);

        // Syntax error
        let (outputs, messages) = kernel.exec("bad ^ # syntax").await?;
        assert_json_is!(messages[0].error_type, "SyntaxError");
        assert_json_is!(messages[0].error_message, "invalid syntax (<code>, line 1)");
        assert!(messages[0].stack_trace.is_some());
        assert_json_is!(outputs, []);

        // Runtime error
        let (outputs, messages) = kernel.exec("foo").await?;
        assert_json_is!(messages[0].error_type, "NameError");
        assert_json_is!(messages[0].error_message, "name 'foo' is not defined");
        assert!(messages[0].stack_trace.is_some());
        assert_json_is!(outputs, []);

        // Set and get another variable
        kernel.set("b", Node::Integer(3)).await?;
        let b = kernel.get("b").await?;
        assert_json_is!(b, 3);

        // Use both variables
        let (outputs, messages) = kernel.exec("a*b").await?;
        assert_json_is!(messages, []);
        assert_json_is!(outputs, [6]);

        Ok(())
    }

    /// Test interrupting a task
    #[tokio::test]
    async fn interrupt() -> Result<()> {
        let mut kernel = match skip_or_kernel().await {
            Ok(kernel) => {
                if kernel.interruptable().await {
                    kernel
                } else {
                    eprintln!("Not interruptable on this OS");
                    return Ok(());
                }
            }
            Err(..) => return Ok(()),
        };

        // Get an "interrupter" so that we can interrupt without doing a double mutable borrow
        let interrupter = MicroKernelInterrupter::new(&kernel)?;

        let task = tokio::task::spawn(async move {
            // Start a long running task in the kernel that should get interrupted in the parent function
            let (outputs, messages) = kernel
                .exec("import time\nstarted = True\ntime.sleep(10)\nfinished = True")
                .await
                .unwrap();
            assert_json_is!(messages[0].error_type, "CodeInterrupt");
            assert_json_is!(outputs, []);

            // Check that was started but not finished
            let (outputs, messages) = kernel
                .exec("[started, 'finished' in locals()]")
                .await
                .unwrap();
            assert_json_is!(messages, []);
            assert_json_is!(outputs, [[true, false]]);
        });

        // Sleep a little to allow the task to start
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        // Interrupt the task and await the results
        interrupter.interrupt();

        // Wait for test results
        task.await?;

        Ok(())
    }

    /// Test forking
    #[tokio::test]
    async fn fork() -> Result<()> {
        let mut kernel = match skip_or_kernel().await {
            Ok(kernel) => {
                if kernel.forkable().await {
                    kernel
                } else {
                    eprintln!("Not forkable on this OS");
                    return Ok(());
                }
            }
            Err(..) => return Ok(()),
        };

        // In the kernel import a module and assign a variable
        let (outputs, messages) = kernel
            .exec("from random import uniform as runif\nvar = runif(0, 1)\nvar")
            .await?;
        assert_json_is!(messages, []);
        assert_eq!(outputs.len(), 1);
        let var = outputs[0].clone();

        // Now fork-exec. The fork should be able to use the module and access the
        // variable but any change to variable should not change its value in the parent kernel
        let (outputs, messages) = kernel.fork_exec("print(var)\nvar = runif(0, 1)").await?;
        assert_json_is!(messages, []);
        assert_eq!(outputs.len(), 1);
        assert_json_is!(outputs[0], var);

        // Back in the parent kernel, var should still have its original value
        assert_json_eq!(var, kernel.get("var").await?);
        let (outputs, messages) = kernel.exec("var").await?;
        assert_json_is!(messages, []);
        assert_eq!(outputs.len(), 1);

        Ok(())
    }
}
