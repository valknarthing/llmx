<p align="center"><code>npm i -g @valknarthing/llmx</code></p>

<p align="center"><strong>LLMX CLI</strong> is a coding agent powered by LiteLLM that runs locally on your computer.
</br>
</br>This project is a community fork with enhanced support for multiple LLM providers via LiteLLM.
</br>Original project: <a href="https://github.com/openai/codex">github.com/openai/codex</a></p>

---

## Quickstart

### Installing and running LLMX CLI

Install globally with npm:

```shell
npm install -g @valknarthing/llmx
```

Then simply run `llmx` to get started:

```shell
llmx
```

<details>
<summary>You can also go to the <a href="https://github.com/valknarthing/llmx/releases/latest">latest GitHub Release</a> and download the appropriate binary for your platform.</summary>

Each GitHub Release contains many executables, but in practice, you likely want one of these:

- macOS
  - Apple Silicon/arm64: `llmx-aarch64-apple-darwin.tar.gz`
  - x86_64 (older Mac hardware): `llmx-x86_64-apple-darwin.tar.gz`
- Linux
  - x86_64: `llmx-x86_64-unknown-linux-musl.tar.gz`
  - arm64: `llmx-aarch64-unknown-linux-musl.tar.gz`

Each archive contains a single entry with the platform baked into the name (e.g., `llmx-x86_64-unknown-linux-musl`), so you likely want to rename it to `llmx` after extracting it.

</details>

### Using LLMX with LiteLLM

LLMX is powered by [LiteLLM](https://docs.litellm.ai/), which provides access to 100+ LLM providers including OpenAI, Anthropic, Google, Azure, AWS Bedrock, and more.

**Quick Start with LiteLLM:**

```bash
# Set your LiteLLM server URL (default: http://localhost:4000/v1)
export LLMX_BASE_URL="http://localhost:4000/v1"
export LLMX_API_KEY="your-api-key"

# Run LLMX
llmx "hello world"
```

**Configuration:** See [LITELLM-SETUP.md](https://github.com/valknarthing/llmx/blob/main/LITELLM-SETUP.md) for detailed setup instructions.

You can also use LLMX with ChatGPT or OpenAI API keys. For authentication options, see the [authentication docs](https://github.com/valknarthing/llmx/blob/main/docs/authentication.md).

### Model Context Protocol (MCP)

LLMX can access MCP servers. To configure them, refer to the [config docs](https://github.com/valknarthing/llmx/blob/main/docs/config.md#mcp_servers).

### Configuration

LLMX CLI supports a rich set of configuration options, with preferences stored in `~/.llmx/config.toml`. For full configuration options, see [Configuration](https://github.com/valknarthing/llmx/blob/main/docs/config.md).

---

### Docs & FAQ

- [**Getting started**](https://github.com/valknarthing/llmx/blob/main/docs/getting-started.md)
  - [CLI usage](https://github.com/valknarthing/llmx/blob/main/docs/getting-started.md#cli-usage)
  - [Slash Commands](https://github.com/valknarthing/llmx/blob/main/docs/slash_commands.md)
  - [Running with a prompt as input](https://github.com/valknarthing/llmx/blob/main/docs/getting-started.md#running-with-a-prompt-as-input)
  - [Example prompts](https://github.com/valknarthing/llmx/blob/main/docs/getting-started.md#example-prompts)
  - [Custom prompts](https://github.com/valknarthing/llmx/blob/main/docs/prompts.md)
  - [Memory with AGENTS.md](https://github.com/valknarthing/llmx/blob/main/docs/getting-started.md#memory-with-agentsmd)
- [**Configuration**](https://github.com/valknarthing/llmx/blob/main/docs/config.md)
  - [Example config](https://github.com/valknarthing/llmx/blob/main/docs/example-config.md)
- [**Sandbox & approvals**](https://github.com/valknarthing/llmx/blob/main/docs/sandbox.md)
- [**Authentication**](https://github.com/valknarthing/llmx/blob/main/docs/authentication.md)
  - [Auth methods](https://github.com/valknarthing/llmx/blob/main/docs/authentication.md#forcing-a-specific-auth-method-advanced)
  - [Login on a "Headless" machine](https://github.com/valknarthing/llmx/blob/main/docs/authentication.md#connecting-on-a-headless-machine)
- **Automating LLMX**
  - [GitHub Action](https://github.com/valknarthing/llmx-action)
  - [TypeScript SDK](https://github.com/valknarthing/llmx/blob/main/sdk/typescript/README.md)
  - [Non-interactive mode (`llmx exec`)](https://github.com/valknarthing/llmx/blob/main/docs/exec.md)
- [**Advanced**](https://github.com/valknarthing/llmx/blob/main/docs/advanced.md)
  - [Tracing / verbose logging](https://github.com/valknarthing/llmx/blob/main/docs/advanced.md#tracing--verbose-logging)
  - [Model Context Protocol (MCP)](https://github.com/valknarthing/llmx/blob/main/docs/advanced.md#model-context-protocol-mcp)
- [**Zero data retention (ZDR)**](https://github.com/valknarthing/llmx/blob/main/docs/zdr.md)
- [**Contributing**](https://github.com/valknarthing/llmx/blob/main/docs/contributing.md)
- [**Install & build**](https://github.com/valknarthing/llmx/blob/main/docs/install.md)
  - [System Requirements](https://github.com/valknarthing/llmx/blob/main/docs/install.md#system-requirements)
  - [DotSlash](https://github.com/valknarthing/llmx/blob/main/docs/install.md#dotslash)
  - [Build from source](https://github.com/valknarthing/llmx/blob/main/docs/install.md#build-from-source)
- [**FAQ**](https://github.com/valknarthing/llmx/blob/main/docs/faq.md)

---

## License

This repository is licensed under the [Apache-2.0 License](LICENSE).
