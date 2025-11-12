use std::path::PathBuf;

use crate::error_code::INVALID_REQUEST_ERROR_CODE;
use crate::llmx_message_processor::LlmxMessageProcessor;
use crate::outgoing_message::OutgoingMessageSender;
use llmx_app_server_protocol::ClientInfo;
use llmx_app_server_protocol::ClientRequest;
use llmx_app_server_protocol::InitializeResponse;

use llmx_app_server_protocol::JSONRPCError;
use llmx_app_server_protocol::JSONRPCErrorError;
use llmx_app_server_protocol::JSONRPCNotification;
use llmx_app_server_protocol::JSONRPCRequest;
use llmx_app_server_protocol::JSONRPCResponse;
use llmx_core::AuthManager;
use llmx_core::ConversationManager;
use llmx_core::config::Config;
use llmx_core::default_client::USER_AGENT_SUFFIX;
use llmx_core::default_client::get_llmx_user_agent;
use llmx_feedback::LlmxFeedback;
use llmx_protocol::protocol::SessionSource;
use std::sync::Arc;

pub(crate) struct MessageProcessor {
    outgoing: Arc<OutgoingMessageSender>,
    llmx_message_processor: LlmxMessageProcessor,
    initialized: bool,
}

impl MessageProcessor {
    /// Create a new `MessageProcessor`, retaining a handle to the outgoing
    /// `Sender` so handlers can enqueue messages to be written to stdout.
    pub(crate) fn new(
        outgoing: OutgoingMessageSender,
        llmx_linux_sandbox_exe: Option<PathBuf>,
        config: Arc<Config>,
        feedback: LlmxFeedback,
    ) -> Self {
        let outgoing = Arc::new(outgoing);
        let auth_manager = AuthManager::shared(
            config.llmx_home.clone(),
            false,
            config.cli_auth_credentials_store_mode,
        );
        let conversation_manager = Arc::new(ConversationManager::new(
            auth_manager.clone(),
            SessionSource::VSCode,
        ));
        let llmx_message_processor = LlmxMessageProcessor::new(
            auth_manager,
            conversation_manager,
            outgoing.clone(),
            llmx_linux_sandbox_exe,
            config,
            feedback,
        );

        Self {
            outgoing,
            llmx_message_processor,
            initialized: false,
        }
    }

    pub(crate) async fn process_request(&mut self, request: JSONRPCRequest) {
        let request_id = request.id.clone();
        let request_json = match serde_json::to_value(&request) {
            Ok(request_json) => request_json,
            Err(err) => {
                let error = JSONRPCErrorError {
                    code: INVALID_REQUEST_ERROR_CODE,
                    message: format!("Invalid request: {err}"),
                    data: None,
                };
                self.outgoing.send_error(request_id, error).await;
                return;
            }
        };

        let llmx_request = match serde_json::from_value::<ClientRequest>(request_json) {
            Ok(llmx_request) => llmx_request,
            Err(err) => {
                let error = JSONRPCErrorError {
                    code: INVALID_REQUEST_ERROR_CODE,
                    message: format!("Invalid request: {err}"),
                    data: None,
                };
                self.outgoing.send_error(request_id, error).await;
                return;
            }
        };

        match llmx_request {
            // Handle Initialize internally so LlmxMessageProcessor does not have to concern
            // itself with the `initialized` bool.
            ClientRequest::Initialize { request_id, params } => {
                if self.initialized {
                    let error = JSONRPCErrorError {
                        code: INVALID_REQUEST_ERROR_CODE,
                        message: "Already initialized".to_string(),
                        data: None,
                    };
                    self.outgoing.send_error(request_id, error).await;
                    return;
                } else {
                    let ClientInfo {
                        name,
                        title: _title,
                        version,
                    } = params.client_info;
                    let user_agent_suffix = format!("{name}; {version}");
                    if let Ok(mut suffix) = USER_AGENT_SUFFIX.lock() {
                        *suffix = Some(user_agent_suffix);
                    }

                    let user_agent = get_llmx_user_agent();
                    let response = InitializeResponse { user_agent };
                    self.outgoing.send_response(request_id, response).await;

                    self.initialized = true;
                    return;
                }
            }
            _ => {
                if !self.initialized {
                    let error = JSONRPCErrorError {
                        code: INVALID_REQUEST_ERROR_CODE,
                        message: "Not initialized".to_string(),
                        data: None,
                    };
                    self.outgoing.send_error(request_id, error).await;
                    return;
                }
            }
        }

        self.llmx_message_processor
            .process_request(llmx_request)
            .await;
    }

    pub(crate) async fn process_notification(&self, notification: JSONRPCNotification) {
        // Currently, we do not expect to receive any notifications from the
        // client, so we just log them.
        tracing::info!("<- notification: {:?}", notification);
    }

    /// Handle a standalone JSON-RPC response originating from the peer.
    pub(crate) async fn process_response(&mut self, response: JSONRPCResponse) {
        tracing::info!("<- response: {:?}", response);
        let JSONRPCResponse { id, result, .. } = response;
        self.outgoing.notify_client_response(id, result).await
    }

    /// Handle an error object received from the peer.
    pub(crate) fn process_error(&mut self, err: JSONRPCError) {
        tracing::error!("<- error: {:?}", err);
    }
}
