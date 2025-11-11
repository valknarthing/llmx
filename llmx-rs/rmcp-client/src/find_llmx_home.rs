use dirs::home_dir;
use std::path::PathBuf;

/// This was copied from llmx-core but llmx-core depends on this crate.
/// TODO: move this to a shared crate lower in the dependency tree.
///
///
/// Returns the path to the LLMX configuration directory, which can be
/// specified by the `LLMX_HOME` environment variable. If not set, defaults to
/// `~/.llmx`.
///
/// - If `LLMX_HOME` is set, the value will be canonicalized and this
///   function will Err if the path does not exist.
/// - If `LLMX_HOME` is not set, this function does not verify that the
///   directory exists.
pub(crate) fn find_llmx_home() -> std::io::Result<PathBuf> {
    // Honor the `LLMX_HOME` environment variable when it is set to allow users
    // (and tests) to override the default location.
    if let Ok(val) = std::env::var("LLMX_HOME")
        && !val.is_empty()
    {
        return PathBuf::from(val).canonicalize();
    }

    let mut p = home_dir().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not find home directory",
        )
    })?;
    p.push(".llmx");
    Ok(p)
}
