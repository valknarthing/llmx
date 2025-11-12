use llmx_app_server::run_main;
use llmx_arg0::arg0_dispatch_or_else;
use llmx_common::CliConfigOverrides;

fn main() -> anyhow::Result<()> {
    arg0_dispatch_or_else(|llmx_linux_sandbox_exe| async move {
        run_main(llmx_linux_sandbox_exe, CliConfigOverrides::default()).await?;
        Ok(())
    })
}
