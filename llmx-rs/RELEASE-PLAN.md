# LLMX Release Plan

## Current Status
- Branch: `feature/rebrand-to-llmx`
- 4 commits ready:
  1. 831e6fa6 - Comprehensive Llmx → LLMX branding (78 files)
  2. 424090f2 - LiteLLM setup documentation
  3. e3507a7f - Fix default provider to litellm ⭐
  4. a88a2f76 - Summary documentation
- Binary: Built and tested ✅
- LiteLLM integration: Working ✅

## Recommended Strategy

### Step 1: Backup Original Main Branch
```bash
# Create a backup tag/branch of original Llmx code
git checkout main
git tag original-llmx-backup
git push origin original-llmx-backup

# Or create a branch
git branch original-llmx-main
git push origin original-llmx-main
```

### Step 2: Merge to Main
```bash
git checkout main
git merge feature/rebrand-to-llmx
git push origin main
```

### Step 3: Create Release Tag
```bash
git tag -a v0.1.0 -m "Initial LLMX release with LiteLLM integration

- Complete rebrand from Llmx to LLMX
- LiteLLM provider support (Chat Completions API)
- Default model: anthropic/claude-sonnet-4-20250514
- Built-in support for multiple LLM providers via LiteLLM
"
git push origin v0.1.0
```

### Step 4: Build for NPM Release

The project has npm packaging scripts in `llmx-cli/scripts/`:
- `build_npm_package.py` - Builds the npm package
- `install_native_deps.py` - Installs native binaries

```bash
# Build the npm package
cd llmx-cli
python3 scripts/build_npm_package.py

# Test locally
npm pack

# Publish to npm (requires npm login)
npm login
npm publish --access public
```

### Step 5: Update Package Metadata

Before publishing, update:

1. **package.json** version:
   ```json
   {
     "name": "@llmx/llmx",
     "version": "0.1.0",
     "description": "LLMX - AI coding assistant with LiteLLM integration"
   }
   ```

2. **README.md** - Update installation instructions:
   ```bash
   npm install -g @llmx/llmx
   ```

## Alternative: Separate Repository

If you want to keep original Llmx intact:

1. **Fork to new repo**: `valknar/llmx` (separate from `valknar/llmx`)
2. Push all changes there
3. Publish from the new repo

## NPM Publishing Checklist

- [ ] npm account ready (@valknar or @llmx org)
- [ ] Package name available (`@llmx/llmx` or `llmx`)
- [ ] Version set in package.json (suggest: 0.1.0)
- [ ] Binary built and tested
- [ ] README updated with new name
- [ ] LICENSE file included
- [ ] .npmignore configured

## Versioning Strategy

Suggest semantic versioning:
- **v0.1.0** - Initial LLMX release (current work)
- **v0.2.0** - Additional features
- **v1.0.0** - Stable release after testing

## Post-Release

1. Create GitHub release with changelog
2. Update documentation
3. Announce on relevant channels
4. Monitor for issues

## Files That Need Version Updates

Before release, update version in:
- `llmx-cli/package.json`
- `llmx-cli/Cargo.toml`
- `llmx-rs/cli/Cargo.toml`
- Root `Cargo.toml` workspace
