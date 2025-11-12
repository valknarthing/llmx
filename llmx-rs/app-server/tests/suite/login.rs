use anyhow::Result;
use app_test_support::McpProcess;
use app_test_support::to_response;
use llmx_app_server_protocol::CancelLoginChatGptParams;
use llmx_app_server_protocol::CancelLoginChatGptResponse;
use llmx_app_server_protocol::GetAuthStatusParams;
use llmx_app_server_protocol::GetAuthStatusResponse;
use llmx_app_server_protocol::JSONRPCError;
use llmx_app_server_protocol::JSONRPCResponse;
use llmx_app_server_protocol::LoginChatGptResponse;
use llmx_app_server_protocol::LogoutChatGptResponse;
use llmx_app_server_protocol::RequestId;
use llmx_core::auth::AuthCredentialsStoreMode;
use llmx_login::login_with_api_key;
use serial_test::serial;
use std::path::Path;
use std::time::Duration;
use tempfile::TempDir;
use tokio::time::timeout;

const DEFAULT_READ_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10);

// Helper to create a config.toml; mirrors create_conversation.rs
fn create_config_toml(llmx_home: &Path) -> std::io::Result<()> {
    let config_toml = llmx_home.join("config.toml");
    std::fs::write(
        config_toml,
        r#"
model = "mock-model"
approval_policy = "never"
sandbox_mode = "danger-full-access"

model_provider = "mock_provider"

[model_providers.mock_provider]
name = "Mock provider for test"
base_url = "http://127.0.0.1:0/v1"
wire_api = "chat"
request_max_retries = 0
stream_max_retries = 0
"#,
    )
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn logout_chatgpt_removes_auth() -> Result<()> {
    let llmx_home = TempDir::new()?;
    create_config_toml(llmx_home.path())?;
    login_with_api_key(
        llmx_home.path(),
        "sk-test-key",
        AuthCredentialsStoreMode::File,
    )?;
    assert!(llmx_home.path().join("auth.json").exists());

    let mut mcp = McpProcess::new_with_env(llmx_home.path(), &[("OPENAI_API_KEY", None)]).await?;
    timeout(DEFAULT_READ_TIMEOUT, mcp.initialize()).await??;

    let id = mcp.send_logout_chat_gpt_request().await?;
    let resp: JSONRPCResponse = timeout(
        DEFAULT_READ_TIMEOUT,
        mcp.read_stream_until_response_message(RequestId::Integer(id)),
    )
    .await??;
    let _ok: LogoutChatGptResponse = to_response(resp)?;

    assert!(
        !llmx_home.path().join("auth.json").exists(),
        "auth.json should be deleted"
    );

    // Verify status reflects signed-out state.
    let status_id = mcp
        .send_get_auth_status_request(GetAuthStatusParams {
            include_token: Some(true),
            refresh_token: Some(false),
        })
        .await?;
    let status_resp: JSONRPCResponse = timeout(
        DEFAULT_READ_TIMEOUT,
        mcp.read_stream_until_response_message(RequestId::Integer(status_id)),
    )
    .await??;
    let status: GetAuthStatusResponse = to_response(status_resp)?;
    assert_eq!(status.auth_method, None);
    assert_eq!(status.auth_token, None);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
// Serialize tests that launch the login server since it binds to a fixed port.
#[serial(login_port)]
async fn login_and_cancel_chatgpt() -> Result<()> {
    let llmx_home = TempDir::new()?;
    create_config_toml(llmx_home.path())?;

    let mut mcp = McpProcess::new(llmx_home.path()).await?;
    timeout(DEFAULT_READ_TIMEOUT, mcp.initialize()).await??;

    let login_id = mcp.send_login_chat_gpt_request().await?;
    let login_resp: JSONRPCResponse = timeout(
        DEFAULT_READ_TIMEOUT,
        mcp.read_stream_until_response_message(RequestId::Integer(login_id)),
    )
    .await??;
    let login: LoginChatGptResponse = to_response(login_resp)?;

    let cancel_id = mcp
        .send_cancel_login_chat_gpt_request(CancelLoginChatGptParams {
            login_id: login.login_id,
        })
        .await?;
    let cancel_resp: JSONRPCResponse = timeout(
        DEFAULT_READ_TIMEOUT,
        mcp.read_stream_until_response_message(RequestId::Integer(cancel_id)),
    )
    .await??;
    let _ok: CancelLoginChatGptResponse = to_response(cancel_resp)?;

    // Optionally observe the completion notification; do not fail if it races.
    let maybe_note = timeout(
        Duration::from_secs(2),
        mcp.read_stream_until_notification_message("llmx/event/login_chat_gpt_complete"),
    )
    .await;
    if maybe_note.is_err() {
        eprintln!("warning: did not observe login_chat_gpt_complete notification after cancel");
    }
    Ok(())
}

fn create_config_toml_forced_login(llmx_home: &Path, forced_method: &str) -> std::io::Result<()> {
    let config_toml = llmx_home.join("config.toml");
    let contents = format!(
        r#"
model = "mock-model"
approval_policy = "never"
sandbox_mode = "danger-full-access"
forced_login_method = "{forced_method}"
"#
    );
    std::fs::write(config_toml, contents)
}

fn create_config_toml_forced_workspace(
    llmx_home: &Path,
    workspace_id: &str,
) -> std::io::Result<()> {
    let config_toml = llmx_home.join("config.toml");
    let contents = format!(
        r#"
model = "mock-model"
approval_policy = "never"
sandbox_mode = "danger-full-access"
forced_chatgpt_workspace_id = "{workspace_id}"
"#
    );
    std::fs::write(config_toml, contents)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn login_chatgpt_rejected_when_forced_api() -> Result<()> {
    let llmx_home = TempDir::new()?;
    create_config_toml_forced_login(llmx_home.path(), "api")?;

    let mut mcp = McpProcess::new(llmx_home.path()).await?;
    timeout(DEFAULT_READ_TIMEOUT, mcp.initialize()).await??;

    let request_id = mcp.send_login_chat_gpt_request().await?;
    let err: JSONRPCError = timeout(
        DEFAULT_READ_TIMEOUT,
        mcp.read_stream_until_error_message(RequestId::Integer(request_id)),
    )
    .await??;

    assert_eq!(
        err.error.message,
        "ChatGPT login is disabled. Use API key login instead."
    );
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
// Serialize tests that launch the login server since it binds to a fixed port.
#[serial(login_port)]
async fn login_chatgpt_includes_forced_workspace_query_param() -> Result<()> {
    let llmx_home = TempDir::new()?;
    create_config_toml_forced_workspace(llmx_home.path(), "ws-forced")?;

    let mut mcp = McpProcess::new(llmx_home.path()).await?;
    timeout(DEFAULT_READ_TIMEOUT, mcp.initialize()).await??;

    let request_id = mcp.send_login_chat_gpt_request().await?;
    let resp: JSONRPCResponse = timeout(
        DEFAULT_READ_TIMEOUT,
        mcp.read_stream_until_response_message(RequestId::Integer(request_id)),
    )
    .await??;

    let login: LoginChatGptResponse = to_response(resp)?;
    assert!(
        login.auth_url.contains("allowed_workspace_id=ws-forced"),
        "auth URL should include forced workspace"
    );
    Ok(())
}
