use std::mem::swap;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use llmx_core::ConversationManager;
use llmx_core::LlmxAuth;
use llmx_core::LlmxConversation;
use llmx_core::ModelProviderInfo;
use llmx_core::built_in_model_providers;
use llmx_core::config::Config;
use llmx_core::features::Feature;
use llmx_core::protocol::AskForApproval;
use llmx_core::protocol::EventMsg;
use llmx_core::protocol::Op;
use llmx_core::protocol::SandboxPolicy;
use llmx_core::protocol::SessionConfiguredEvent;
use llmx_protocol::config_types::ReasoningSummary;
use llmx_protocol::user_input::UserInput;
use serde_json::Value;
use tempfile::TempDir;
use wiremock::MockServer;

use crate::load_default_config_for_test;
use crate::responses::start_mock_server;
use crate::wait_for_event;

type ConfigMutator = dyn FnOnce(&mut Config) + Send;

pub struct TestLlmxBuilder {
    config_mutators: Vec<Box<ConfigMutator>>,
}

impl TestLlmxBuilder {
    pub fn with_config<T>(mut self, mutator: T) -> Self
    where
        T: FnOnce(&mut Config) + Send + 'static,
    {
        self.config_mutators.push(Box::new(mutator));
        self
    }

    pub async fn build(&mut self, server: &wiremock::MockServer) -> anyhow::Result<TestLlmx> {
        let home = Arc::new(TempDir::new()?);
        self.build_with_home(server, home, None).await
    }

    pub async fn resume(
        &mut self,
        server: &wiremock::MockServer,
        home: Arc<TempDir>,
        rollout_path: PathBuf,
    ) -> anyhow::Result<TestLlmx> {
        self.build_with_home(server, home, Some(rollout_path)).await
    }

    async fn build_with_home(
        &mut self,
        server: &wiremock::MockServer,
        home: Arc<TempDir>,
        resume_from: Option<PathBuf>,
    ) -> anyhow::Result<TestLlmx> {
        let (config, cwd) = self.prepare_config(server, &home).await?;
        let conversation_manager = ConversationManager::with_auth(LlmxAuth::from_api_key("dummy"));

        let new_conversation = match resume_from {
            Some(path) => {
                let auth_manager =
                    llmx_core::AuthManager::from_auth_for_testing(LlmxAuth::from_api_key("dummy"));
                conversation_manager
                    .resume_conversation_from_rollout(config, path, auth_manager)
                    .await?
            }
            None => conversation_manager.new_conversation(config).await?,
        };

        Ok(TestLlmx {
            home,
            cwd,
            llmx: new_conversation.conversation,
            session_configured: new_conversation.session_configured,
        })
    }

    async fn prepare_config(
        &mut self,
        server: &wiremock::MockServer,
        home: &TempDir,
    ) -> anyhow::Result<(Config, Arc<TempDir>)> {
        let model_provider = ModelProviderInfo {
            base_url: Some(format!("{}/v1", server.uri())),
            ..built_in_model_providers()["openai"].clone()
        };
        let cwd = Arc::new(TempDir::new()?);
        let mut config = load_default_config_for_test(home);
        config.cwd = cwd.path().to_path_buf();
        config.model_provider = model_provider;
        if let Ok(cmd) = assert_cmd::Command::cargo_bin("llmx") {
            config.llmx_linux_sandbox_exe = Some(PathBuf::from(cmd.get_program().to_os_string()));
        }

        let mut mutators = vec![];
        swap(&mut self.config_mutators, &mut mutators);
        for mutator in mutators {
            mutator(&mut config);
        }

        if config.include_apply_patch_tool {
            config.features.enable(Feature::ApplyPatchFreeform);
        } else {
            config.features.disable(Feature::ApplyPatchFreeform);
        }

        Ok((config, cwd))
    }
}

pub struct TestLlmx {
    pub home: Arc<TempDir>,
    pub cwd: Arc<TempDir>,
    pub llmx: Arc<LlmxConversation>,
    pub session_configured: SessionConfiguredEvent,
}

impl TestLlmx {
    pub fn cwd_path(&self) -> &Path {
        self.cwd.path()
    }

    pub fn workspace_path(&self, rel: impl AsRef<Path>) -> PathBuf {
        self.cwd_path().join(rel)
    }

    pub async fn submit_turn(&self, prompt: &str) -> Result<()> {
        self.submit_turn_with_policy(prompt, SandboxPolicy::DangerFullAccess)
            .await
    }

    pub async fn submit_turn_with_policy(
        &self,
        prompt: &str,
        sandbox_policy: SandboxPolicy,
    ) -> Result<()> {
        let session_model = self.session_configured.model.clone();
        self.llmx
            .submit(Op::UserTurn {
                items: vec![UserInput::Text {
                    text: prompt.into(),
                }],
                final_output_json_schema: None,
                cwd: self.cwd.path().to_path_buf(),
                approval_policy: AskForApproval::Never,
                sandbox_policy,
                model: session_model,
                effort: None,
                summary: ReasoningSummary::Auto,
            })
            .await?;

        wait_for_event(&self.llmx, |event| {
            matches!(event, EventMsg::TaskComplete(_))
        })
        .await;
        Ok(())
    }
}

pub struct TestLlmxHarness {
    server: MockServer,
    test: TestLlmx,
}

impl TestLlmxHarness {
    pub async fn new() -> Result<Self> {
        Self::with_builder(test_llmx()).await
    }

    pub async fn with_config(mutator: impl FnOnce(&mut Config) + Send + 'static) -> Result<Self> {
        Self::with_builder(test_llmx().with_config(mutator)).await
    }

    pub async fn with_builder(mut builder: TestLlmxBuilder) -> Result<Self> {
        let server = start_mock_server().await;
        let test = builder.build(&server).await?;
        Ok(Self { server, test })
    }

    pub fn server(&self) -> &MockServer {
        &self.server
    }

    pub fn test(&self) -> &TestLlmx {
        &self.test
    }

    pub fn cwd(&self) -> &Path {
        self.test.cwd_path()
    }

    pub fn path(&self, rel: impl AsRef<Path>) -> PathBuf {
        self.test.workspace_path(rel)
    }

    pub async fn submit(&self, prompt: &str) -> Result<()> {
        self.test.submit_turn(prompt).await
    }

    pub async fn submit_with_policy(
        &self,
        prompt: &str,
        sandbox_policy: SandboxPolicy,
    ) -> Result<()> {
        self.test
            .submit_turn_with_policy(prompt, sandbox_policy)
            .await
    }

    pub async fn request_bodies(&self) -> Vec<Value> {
        self.server
            .received_requests()
            .await
            .expect("requests")
            .into_iter()
            .map(|req| serde_json::from_slice(&req.body).expect("request body json"))
            .collect()
    }

    pub async fn function_call_output_value(&self, call_id: &str) -> Value {
        let bodies = self.request_bodies().await;
        function_call_output(&bodies, call_id).clone()
    }

    pub async fn function_call_stdout(&self, call_id: &str) -> String {
        self.function_call_output_value(call_id)
            .await
            .get("output")
            .and_then(Value::as_str)
            .expect("output string")
            .to_string()
    }

    pub async fn custom_tool_call_output(&self, call_id: &str) -> String {
        let bodies = self.request_bodies().await;
        custom_tool_call_output(&bodies, call_id)
            .get("output")
            .and_then(Value::as_str)
            .expect("output string")
            .to_string()
    }
}

fn custom_tool_call_output<'a>(bodies: &'a [Value], call_id: &str) -> &'a Value {
    for body in bodies {
        if let Some(items) = body.get("input").and_then(Value::as_array) {
            for item in items {
                if item.get("type").and_then(Value::as_str) == Some("custom_tool_call_output")
                    && item.get("call_id").and_then(Value::as_str) == Some(call_id)
                {
                    return item;
                }
            }
        }
    }
    panic!("custom_tool_call_output {call_id} not found");
}

fn function_call_output<'a>(bodies: &'a [Value], call_id: &str) -> &'a Value {
    for body in bodies {
        if let Some(items) = body.get("input").and_then(Value::as_array) {
            for item in items {
                if item.get("type").and_then(Value::as_str) == Some("function_call_output")
                    && item.get("call_id").and_then(Value::as_str) == Some(call_id)
                {
                    return item;
                }
            }
        }
    }
    panic!("function_call_output {call_id} not found");
}

pub fn test_llmx() -> TestLlmxBuilder {
    TestLlmxBuilder {
        config_mutators: vec![],
    }
}
