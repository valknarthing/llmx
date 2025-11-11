#!/bin/bash

# Set "chatgpt.cliExecutable": "/Users/<USERNAME>/code/llmx/scripts/debug-llmx.sh" in VSCode settings to always get the
# latest llmx-rs binary when debugging LLMX Extension.


set -euo pipefail

LLMX_RS_DIR=$(realpath "$(dirname "$0")/../llmx-rs")
(cd "$LLMX_RS_DIR" && cargo run --quiet --bin llmx -- "$@")