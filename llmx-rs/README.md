# LLMX CLI (Rust Implementation)

We provide LLMX CLI as a standalone, native executable to ensure a zero-dependency install.

## Installing LLMX

Today, the easiest way to install LLMX is via `npm`:

```shell
npm i -g @llmx/llmx
llmx
```

You can also install via Homebrew (`brew install --cask llmx`) or download a platform-specific release directly from our [GitHub Releases](https://github.com/valknar/llmx/releases).

## Documentation quickstart

- First run with LLMX? Follow the walkthrough in [`docs/getting-started.md`](../docs/getting-started.md) for prompts, keyboard shortcuts, and session management.
- Already shipping with LLMX and want deeper control? Jump to [`docs/advanced.md`](../docs/advanced.md) and the configuration reference at [`docs/config.md`](../docs/config.md).

## What's new in the Rust CLI

The Rust implementation is now the maintained LLMX CLI and serves as the default experience. It includes a number of features that the legacy TypeScript CLI never supported.

### Config

LLMX supports a rich set of configuration options. Note that the Rust CLI uses `config.toml` instead of `config.json`. See [`docs/config.md`](../docs/config.md) for details.

### Model Context Protocol Support

#### MCP client

LLMX CLI functions as an MCP client that allows the LLMX CLI and IDE extension to connect to MCP servers on startup. See the [`configuration documentation`](../docs/config.md#mcp_servers) for details.

#### MCP server (experimental)

LLMX can be launched as an MCP _server_ by running `llmx mcp-server`. This allows _other_ MCP clients to use LLMX as a tool for another agent.

Use the [`@modelcontextprotocol/inspector`](https://github.com/modelcontextprotocol/inspector) to try it out:

```shell
npx @modelcontextprotocol/inspector llmx mcp-server
```

Use `llmx mcp` to add/list/get/remove MCP server launchers defined in `config.toml`, and `llmx mcp-server` to run the MCP server directly.

### Notifications

You can enable notifications by configuring a script that is run whenever the agent finishes a turn. The [notify documentation](../docs/config.md#notify) includes a detailed example that explains how to get desktop notifications via [terminal-notifier](https://github.com/julienXX/terminal-notifier) on macOS.

### `llmx exec` to run LLMX programmatically/non-interactively

To run LLMX non-interactively, run `llmx exec PROMPT` (you can also pass the prompt via `stdin`) and LLMX will work on your task until it decides that it is done and exits. Output is printed to the terminal directly. You can set the `RUST_LOG` environment variable to see more about what's going on.

### Experimenting with the LLMX Sandbox

To test to see what happens when a command is run under the sandbox provided by LLMX, we provide the following subcommands in LLMX CLI:

```
# macOS
llmx sandbox macos [--full-auto] [--log-denials] [COMMAND]...

# Linux
llmx sandbox linux [--full-auto] [COMMAND]...

# Windows
llmx sandbox windows [--full-auto] [COMMAND]...

# Legacy aliases
llmx debug seatbelt [--full-auto] [--log-denials] [COMMAND]...
llmx debug landlock [--full-auto] [COMMAND]...
```

### Selecting a sandbox policy via `--sandbox`

The Rust CLI exposes a dedicated `--sandbox` (`-s`) flag that lets you pick the sandbox policy **without** having to reach for the generic `-c/--config` option:

```shell
# Run LLMX with the default, read-only sandbox
llmx --sandbox read-only

# Allow the agent to write within the current workspace while still blocking network access
llmx --sandbox workspace-write

# Danger! Disable sandboxing entirely (only do this if you are already running in a container or other isolated env)
llmx --sandbox danger-full-access
```

The same setting can be persisted in `~/.llmx/config.toml` via the top-level `sandbox_mode = "MODE"` key, e.g. `sandbox_mode = "workspace-write"`.

## Code Organization

This folder is the root of a Cargo workspace. It contains quite a bit of experimental code, but here are the key crates:

- [`core/`](./core) contains the business logic for LLMX. Ultimately, we hope this to be a library crate that is generally useful for building other Rust/native applications that use LLMX.
- [`exec/`](./exec) "headless" CLI for use in automation.
- [`tui/`](./tui) CLI that launches a fullscreen TUI built with [Ratatui](https://ratatui.rs/).
- [`cli/`](./cli) CLI multitool that provides the aforementioned CLIs via subcommands.
