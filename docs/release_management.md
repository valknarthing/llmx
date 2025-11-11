# Release Management

Currently, we made LLMX binaries available in three places:

- GitHub Releases https://github.com/valknar/llmx/releases/
- `@llmx/llmx` on npm: https://www.npmjs.com/package/@llmx/llmx
- `llmx` on Homebrew: https://formulae.brew.sh/cask/llmx

# Cutting a Release

Run the `llmx-rs/scripts/create_github_release` script in the repository to publish a new release. The script will choose the appropriate version number depending on the type of release you are creating.

To cut a new alpha release from `main` (feel free to cut alphas liberally):

```
./llmx-rs/scripts/create_github_release --publish-alpha
```

To cut a new _public_ release from `main` (which requires more caution), run:

```
./llmx-rs/scripts/create_github_release --publish-release
```

TIP: Add the `--dry-run` flag to report the next version number for the respective release and exit.

Running the publishing script will kick off a GitHub Action to build the release, so go to https://github.com/valknar/llmx/actions/workflows/rust-release.yml to find the corresponding workflow. (Note: we should automate finding the workflow URL with `gh`.)

When the workflow finishes, the GitHub Release is "done," but you still have to consider npm and Homebrew.

## Publishing to npm

The GitHub Action is responsible for publishing to npm.

## Publishing to Homebrew

For Homebrew, we ship LLMX as a cask. Homebrew's automation system checks our GitHub repo every few hours for a new release and will open a PR to update the cask with the latest binary.

Inevitably, you just have to refresh this page periodically to see if the release has been picked up by their automation system:

https://github.com/Homebrew/homebrew-cask/pulls?q=%3Apr+llmx

For reference, our Homebrew cask lives at:

https://github.com/Homebrew/homebrew-cask/blob/main/Casks/c/llmx.rb
