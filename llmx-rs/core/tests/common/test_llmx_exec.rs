#![allow(clippy::expect_used)]
use llmx_core::auth::LLMX_API_KEY_ENV_VAR;
use std::path::Path;
use tempfile::TempDir;
use wiremock::MockServer;

pub struct TestLlmxExecBuilder {
    home: TempDir,
    cwd: TempDir,
}

impl TestLlmxExecBuilder {
    pub fn cmd(&self) -> assert_cmd::Command {
        let mut cmd =
            assert_cmd::Command::cargo_bin("llmx-exec").expect("should find binary for llmx-exec");
        cmd.current_dir(self.cwd.path())
            .env("LLMX_HOME", self.home.path())
            .env(LLMX_API_KEY_ENV_VAR, "dummy");
        cmd
    }
    pub fn cmd_with_server(&self, server: &MockServer) -> assert_cmd::Command {
        let mut cmd = self.cmd();
        let base = format!("{}/v1", server.uri());
        cmd.env("LLMX_BASE_URL", base);
        cmd
    }

    pub fn cwd_path(&self) -> &Path {
        self.cwd.path()
    }
    pub fn home_path(&self) -> &Path {
        self.home.path()
    }
}

pub fn test_llmx_exec() -> TestLlmxExecBuilder {
    TestLlmxExecBuilder {
        home: TempDir::new().expect("create temp home"),
        cwd: TempDir::new().expect("create temp cwd"),
    }
}
