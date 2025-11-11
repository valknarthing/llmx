# llmx-linux-sandbox

This crate is responsible for producing:

- a `llmx-linux-sandbox` standalone executable for Linux that is bundled with the Node.js version of the LLMX CLI
- a lib crate that exposes the business logic of the executable as `run_main()` so that
  - the `llmx-exec` CLI can check if its arg0 is `llmx-linux-sandbox` and, if so, execute as if it were `llmx-linux-sandbox`
  - this should also be true of the `llmx` multitool CLI
