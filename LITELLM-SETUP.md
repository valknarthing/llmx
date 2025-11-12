# LLMX with LiteLLM Configuration Guide

## Quick Start

### 1. Set Environment Variables

```bash
export LLMX_BASE_URL="https://llm.ai.pivoine.art/v1"
export LLMX_API_KEY="your-litellm-master-key"
```

### 2. Create Configuration File

Create `~/.llmx/config.toml`:

```toml
model_provider = "litellm"
model = "anthropic/claude-sonnet-4-20250514"
```

### 3. Run LLMX

```bash
# Use default config
llmx "hello world"

# Override model
llmx -m "openai/gpt-4" "hello world"

# Override provider and model
llmx -c model_provider=litellm -m "anthropic/claude-sonnet-4-20250514" "hello"
```

## Important Notes

### DO NOT use provider prefix in model name

❌ Wrong: `llmx -m "litellm:anthropic/claude-sonnet-4-20250514"`
✅ Correct: `llmx -c model_provider=litellm -m "anthropic/claude-sonnet-4-20250514"`

LLMX uses separate provider and model parameters, not a combined `provider:model` syntax.

### Provider Selection

The provider determines which API endpoint and format to use:

- `litellm` → Uses Chat Completions API (`/v1/chat/completions`)
- `openai` → Uses Responses API (`/v1/responses`) - NOT compatible with LiteLLM

### Model Names

LiteLLM uses `provider/model` format:

- `anthropic/claude-sonnet-4-20250514`
- `openai/gpt-4`
- `openai/gpt-4o`

Check your LiteLLM configuration for available models.

## Troubleshooting

### Error: "prompt_cache_key: Extra inputs are not permitted"

**Cause**: Using wrong provider (defaults to OpenAI which uses Responses API)
**Fix**: Add `-c model_provider=litellm` or set `model_provider = "litellm"` in config

### Error: "Invalid model name passed in model=litellm:..."

**Cause**: Including provider prefix in model name
**Fix**: Remove the `litellm:` prefix, use just the model name

### Error: "Model provider `litellm` not found"

**Cause**: Using old binary without LiteLLM provider
**Fix**: Use the newly built binary at `llmx-rs/target/release/llmx`

## Binary Location

Latest binary with LiteLLM support:

```
/home/valknar/Projects/llmx/llmx/llmx-rs/target/release/llmx
```
