use std::sync::Arc;

use core_test_support::load_default_config_for_test;
use core_test_support::responses;
use futures::StreamExt;
use llmx_app_server_protocol::AuthMode;
use llmx_core::ContentItem;
use llmx_core::ModelClient;
use llmx_core::ModelProviderInfo;
use llmx_core::Prompt;
use llmx_core::ResponseEvent;
use llmx_core::ResponseItem;
use llmx_core::WireApi;
use llmx_otel::otel_event_manager::OtelEventManager;
use llmx_protocol::ConversationId;
use llmx_protocol::protocol::SessionSource;
use tempfile::TempDir;
use wiremock::matchers::header;

#[tokio::test]
async fn responses_stream_includes_subagent_header_on_review() {
    core_test_support::skip_if_no_network!();

    let server = responses::start_mock_server().await;
    let response_body = responses::sse(vec![
        responses::ev_response_created("resp-1"),
        responses::ev_completed("resp-1"),
    ]);

    let request_recorder = responses::mount_sse_once_match(
        &server,
        header("x-openai-subagent", "review"),
        response_body,
    )
    .await;

    let provider = ModelProviderInfo {
        name: "mock".into(),
        base_url: Some(format!("{}/v1", server.uri())),
        env_key: None,
        env_key_instructions: None,
        experimental_bearer_token: None,
        wire_api: WireApi::Responses,
        query_params: None,
        http_headers: None,
        env_http_headers: None,
        request_max_retries: Some(0),
        stream_max_retries: Some(0),
        stream_idle_timeout_ms: Some(5_000),
        requires_openai_auth: false,
    };

    let llmx_home = TempDir::new().expect("failed to create TempDir");
    let mut config = load_default_config_for_test(&llmx_home);
    config.model_provider_id = provider.name.clone();
    config.model_provider = provider.clone();
    let effort = config.model_reasoning_effort;
    let summary = config.model_reasoning_summary;
    let config = Arc::new(config);

    let conversation_id = ConversationId::new();

    let otel_event_manager = OtelEventManager::new(
        conversation_id,
        config.model.as_str(),
        config.model_family.slug.as_str(),
        None,
        Some("test@test.com".to_string()),
        Some(AuthMode::ChatGPT),
        false,
        "test".to_string(),
    );

    let client = ModelClient::new(
        Arc::clone(&config),
        None,
        otel_event_manager,
        provider,
        effort,
        summary,
        conversation_id,
        SessionSource::SubAgent(llmx_protocol::protocol::SubAgentSource::Review),
    );

    let mut prompt = Prompt::default();
    prompt.input = vec![ResponseItem::Message {
        id: None,
        role: "user".into(),
        content: vec![ContentItem::InputText {
            text: "hello".into(),
        }],
    }];

    let mut stream = client.stream(&prompt).await.expect("stream failed");
    while let Some(event) = stream.next().await {
        if matches!(event, Ok(ResponseEvent::Completed { .. })) {
            break;
        }
    }

    let request = request_recorder.single_request();
    assert_eq!(
        request.header("x-openai-subagent").as_deref(),
        Some("review")
    );
}

#[tokio::test]
async fn responses_stream_includes_subagent_header_on_other() {
    core_test_support::skip_if_no_network!();

    let server = responses::start_mock_server().await;
    let response_body = responses::sse(vec![
        responses::ev_response_created("resp-1"),
        responses::ev_completed("resp-1"),
    ]);

    let request_recorder = responses::mount_sse_once_match(
        &server,
        header("x-openai-subagent", "my-task"),
        response_body,
    )
    .await;

    let provider = ModelProviderInfo {
        name: "mock".into(),
        base_url: Some(format!("{}/v1", server.uri())),
        env_key: None,
        env_key_instructions: None,
        experimental_bearer_token: None,
        wire_api: WireApi::Responses,
        query_params: None,
        http_headers: None,
        env_http_headers: None,
        request_max_retries: Some(0),
        stream_max_retries: Some(0),
        stream_idle_timeout_ms: Some(5_000),
        requires_openai_auth: false,
    };

    let llmx_home = TempDir::new().expect("failed to create TempDir");
    let mut config = load_default_config_for_test(&llmx_home);
    config.model_provider_id = provider.name.clone();
    config.model_provider = provider.clone();
    let effort = config.model_reasoning_effort;
    let summary = config.model_reasoning_summary;
    let config = Arc::new(config);

    let conversation_id = ConversationId::new();

    let otel_event_manager = OtelEventManager::new(
        conversation_id,
        config.model.as_str(),
        config.model_family.slug.as_str(),
        None,
        Some("test@test.com".to_string()),
        Some(AuthMode::ChatGPT),
        false,
        "test".to_string(),
    );

    let client = ModelClient::new(
        Arc::clone(&config),
        None,
        otel_event_manager,
        provider,
        effort,
        summary,
        conversation_id,
        SessionSource::SubAgent(llmx_protocol::protocol::SubAgentSource::Other(
            "my-task".to_string(),
        )),
    );

    let mut prompt = Prompt::default();
    prompt.input = vec![ResponseItem::Message {
        id: None,
        role: "user".into(),
        content: vec![ContentItem::InputText {
            text: "hello".into(),
        }],
    }];

    let mut stream = client.stream(&prompt).await.expect("stream failed");
    while let Some(event) = stream.next().await {
        if matches!(event, Ok(ResponseEvent::Completed { .. })) {
            break;
        }
    }

    let request = request_recorder.single_request();
    assert_eq!(
        request.header("x-openai-subagent").as_deref(),
        Some("my-task")
    );
}
