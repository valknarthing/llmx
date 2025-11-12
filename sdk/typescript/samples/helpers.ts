import path from "node:path";

export function llmxPathOverride() {
  return (
    process.env.LLMX_EXECUTABLE ??
    path.join(process.cwd(), "..", "..", "llmx-rs", "target", "debug", "llmx")
  );
}
