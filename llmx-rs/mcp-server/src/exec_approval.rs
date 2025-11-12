use std::path::PathBuf;
use std::sync::Arc;

use llmx_core::LlmxConversation;
use llmx_core::protocol::Op;
use llmx_core::protocol::ReviewDecision;
use llmx_core::protocol::SandboxCommandAssessment;
use llmx_protocol::parse_command::ParsedCommand;
use mcp_types::ElicitRequest;
use mcp_types::ElicitRequestParamsRequestedSchema;
use mcp_types::JSONRPCErrorError;
use mcp_types::ModelContextProtocolRequest;
use mcp_types::RequestId;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use tracing::error;

use crate::llmx_tool_runner::INVALID_PARAMS_ERROR_CODE;

/// Conforms to [`mcp_types::ElicitRequestParams`] so that it can be used as the
/// `params` field of an [`ElicitRequest`].
#[derive(Debug, Deserialize, Serialize)]
pub struct ExecApprovalElicitRequestParams {
    // These fields are required so that `params`
    // conforms to ElicitRequestParams.
    pub message: String,

    #[serde(rename = "requestedSchema")]
    pub requested_schema: ElicitRequestParamsRequestedSchema,

    // These are additional fields the client can use to
    // correlate the request with the llmx tool call.
    pub llmx_elicitation: String,
    pub llmx_mcp_tool_call_id: String,
    pub llmx_event_id: String,
    pub llmx_call_id: String,
    pub llmx_command: Vec<String>,
    pub llmx_cwd: PathBuf,
    pub llmx_parsed_cmd: Vec<ParsedCommand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llmx_risk: Option<SandboxCommandAssessment>,
}

// TODO(mbolin): ExecApprovalResponse does not conform to ElicitResult. See:
// - https://github.com/modelcontextprotocol/modelcontextprotocol/blob/f962dc1780fa5eed7fb7c8a0232f1fc83ef220cd/schema/2025-06-18/schema.json#L617-L636
// - https://modelcontextprotocol.io/specification/draft/client/elicitation#protocol-messages
// It should have "action" and "content" fields.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExecApprovalResponse {
    pub decision: ReviewDecision,
}

#[allow(clippy::too_many_arguments)]
pub(crate) async fn handle_exec_approval_request(
    command: Vec<String>,
    cwd: PathBuf,
    outgoing: Arc<crate::outgoing_message::OutgoingMessageSender>,
    llmx: Arc<LlmxConversation>,
    request_id: RequestId,
    tool_call_id: String,
    event_id: String,
    call_id: String,
    llmx_parsed_cmd: Vec<ParsedCommand>,
    llmx_risk: Option<SandboxCommandAssessment>,
) {
    let escaped_command =
        shlex::try_join(command.iter().map(String::as_str)).unwrap_or_else(|_| command.join(" "));
    let message = format!(
        "Allow LLMX to run `{escaped_command}` in `{cwd}`?",
        cwd = cwd.to_string_lossy()
    );

    let params = ExecApprovalElicitRequestParams {
        message,
        requested_schema: ElicitRequestParamsRequestedSchema {
            r#type: "object".to_string(),
            properties: json!({}),
            required: None,
        },
        llmx_elicitation: "exec-approval".to_string(),
        llmx_mcp_tool_call_id: tool_call_id.clone(),
        llmx_event_id: event_id.clone(),
        llmx_call_id: call_id,
        llmx_command: command,
        llmx_cwd: cwd,
        llmx_parsed_cmd,
        llmx_risk,
    };
    let params_json = match serde_json::to_value(&params) {
        Ok(value) => value,
        Err(err) => {
            let message = format!("Failed to serialize ExecApprovalElicitRequestParams: {err}");
            error!("{message}");

            outgoing
                .send_error(
                    request_id.clone(),
                    JSONRPCErrorError {
                        code: INVALID_PARAMS_ERROR_CODE,
                        message,
                        data: None,
                    },
                )
                .await;

            return;
        }
    };

    let on_response = outgoing
        .send_request(ElicitRequest::METHOD, Some(params_json))
        .await;

    // Listen for the response on a separate task so we don't block the main agent loop.
    {
        let llmx = llmx.clone();
        let event_id = event_id.clone();
        tokio::spawn(async move {
            on_exec_approval_response(event_id, on_response, llmx).await;
        });
    }
}

async fn on_exec_approval_response(
    event_id: String,
    receiver: tokio::sync::oneshot::Receiver<mcp_types::Result>,
    llmx: Arc<LlmxConversation>,
) {
    let response = receiver.await;
    let value = match response {
        Ok(value) => value,
        Err(err) => {
            error!("request failed: {err:?}");
            return;
        }
    };

    // Try to deserialize `value` and then make the appropriate call to `llmx`.
    let response = serde_json::from_value::<ExecApprovalResponse>(value).unwrap_or_else(|err| {
        error!("failed to deserialize ExecApprovalResponse: {err}");
        // If we cannot deserialize the response, we deny the request to be
        // conservative.
        ExecApprovalResponse {
            decision: ReviewDecision::Denied,
        }
    });

    if let Err(err) = llmx
        .submit(Op::ExecApproval {
            id: event_id,
            decision: response.decision,
        })
        .await
    {
        error!("failed to submit ExecApproval: {err}");
    }
}
