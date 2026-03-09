//! Types for the Copilot SDK protocol, derived from the official Go SDK.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Session configuration for creating a new session
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SessionConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_message: Option<SystemMessageConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_tools: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_tools: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<String>,
    /// Always request permission callback
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_permission: Option<bool>,
}

/// System message configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemMessageConfig {
    /// "append" or "replace"
    pub mode: String,
    pub content: String,
}

/// Tool definition sent to the server during session creation
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ToolDefinition {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides_built_in_tool: Option<bool>,
}

/// Response from session.create
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSessionResponse {
    pub session_id: String,
    #[serde(default)]
    #[allow(dead_code)]
    pub workspace_path: Option<String>,
}

/// Request for session.send
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionSendRequest {
    pub session_id: String,
    pub prompt: String,
}

/// Response from session.send
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct SessionSendResponse {
    pub message_id: String,
}

/// Session event received via the session.event notification
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SessionEventNotification {
    pub session_id: String,
    pub event: SessionEvent,
}

/// A session event
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SessionEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(default)]
    pub data: SessionEventData,
}

/// Data payload of a session event
#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SessionEventData {
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub delta_content: Option<String>,
    #[serde(default)]
    pub tool_name: Option<String>,
    /// Catch-all for fields we don't explicitly model
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Event type constants matching the official SDK
pub const ASSISTANT_MESSAGE: &str = "assistant.message";
pub const ASSISTANT_TURN_END: &str = "assistant.turn_end";
pub const SESSION_IDLE: &str = "session.idle";
pub const SESSION_ERROR: &str = "session.error";

/// Tool call request from the server
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolCallRequest {
    #[allow(dead_code)]
    pub session_id: String,
    #[allow(dead_code)]
    pub tool_call_id: String,
    pub tool_name: String,
    #[serde(default)]
    pub arguments: Value,
}

/// Tool result returned to the server
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolCallResponse {
    pub result: ToolResult,
}

/// Result of a tool invocation
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ToolResult {
    pub text_result_for_llm: String,
    pub result_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl ToolResult {
    pub fn success(text: String) -> Self {
        Self {
            text_result_for_llm: text,
            result_type: "success".to_string(),
            error: None,
        }
    }

    pub fn failure(error: String) -> Self {
        Self {
            text_result_for_llm: format!("Error: {}", error),
            result_type: "failure".to_string(),
            error: Some(error),
        }
    }
}

/// Permission request from the server
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct PermissionRequestPayload {
    pub session_id: String,
    pub request: Value,
}

/// Permission response sent back to the server
#[derive(Debug, Serialize)]
pub struct PermissionResponse {
    pub result: PermissionResult,
}

/// Permission result
#[derive(Debug, Serialize)]
pub struct PermissionResult {
    pub kind: String,
}

impl PermissionResult {
    pub fn approve() -> Self {
        Self {
            kind: "approved".to_string(),
        }
    }

    pub fn deny() -> Self {
        Self {
            kind: "denied".to_string(),
        }
    }
}

/// Ping request
#[derive(Debug, Serialize)]
pub struct PingRequest {
    pub message: String,
}

/// Ping response
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PingResponse {
    #[serde(default)]
    #[allow(dead_code)]
    pub message: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    pub timestamp: Option<i64>,
    #[serde(default)]
    pub protocol_version: Option<i32>,
}

/// Model info
#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct ModelInfo {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
}

/// SDK protocol version — must match the server
pub const SDK_PROTOCOL_VERSION: i32 = 3;

/// v3 broadcast event: custom tool invoked (we must respond via session.tools.handlePendingToolCall)
pub const EXTERNAL_TOOL_REQUESTED: &str = "external_tool.requested";
