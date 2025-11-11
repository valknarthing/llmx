# llmx-core

This crate implements the business logic for LLMX. It is designed to be used by the various LLMX UIs written in Rust.

## Dependencies

Note that `llmx-core` makes some assumptions about certain helper utilities being available in the environment. Currently, this support matrix is:

### macOS

Expects `/usr/bin/sandbox-exec` to be present.

### Linux

Expects the binary containing `llmx-core` to run the equivalent of `llmx sandbox linux` (legacy alias: `llmx debug landlock`) when `arg0` is `llmx-linux-sandbox`. See the `llmx-arg0` crate for details.

### All Platforms

Expects the binary containing `llmx-core` to simulate the virtual `apply_patch` CLI when `arg1` is `--llmx-run-as-apply-patch`. See the `llmx-arg0` crate for details.
