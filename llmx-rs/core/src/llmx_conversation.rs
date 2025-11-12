use crate::error::Result as LlmxResult;
use crate::llmx::Llmx;
use crate::protocol::Event;
use crate::protocol::Op;
use crate::protocol::Submission;
use std::path::PathBuf;

pub struct LlmxConversation {
    llmx: Llmx,
    rollout_path: PathBuf,
}

/// Conduit for the bidirectional stream of messages that compose a conversation
/// in Llmx.
impl LlmxConversation {
    pub(crate) fn new(llmx: Llmx, rollout_path: PathBuf) -> Self {
        Self { llmx, rollout_path }
    }

    pub async fn submit(&self, op: Op) -> LlmxResult<String> {
        self.llmx.submit(op).await
    }

    /// Use sparingly: this is intended to be removed soon.
    pub async fn submit_with_id(&self, sub: Submission) -> LlmxResult<()> {
        self.llmx.submit_with_id(sub).await
    }

    pub async fn next_event(&self) -> LlmxResult<Event> {
        self.llmx.next_event().await
    }

    pub fn rollout_path(&self) -> PathBuf {
        self.rollout_path.clone()
    }
}
