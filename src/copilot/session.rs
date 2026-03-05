//! Session management — send messages, handle events, tool dispatch.

use super::jsonrpc::Transport;
use super::types::*;
use anyhow::{Context, Result};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// A tool handler function: receives arguments, returns a ToolResult
pub type ToolHandler = Arc<dyn Fn(Value) -> ToolResult + Send + Sync>;

/// A Copilot session
pub struct Session {
    transport: Arc<Transport>,
    pub session_id: String,
    tool_handlers: Arc<Mutex<HashMap<String, ToolHandler>>>,
    /// The workspace root, used to relativize and filter tool call paths in logs
    root: Option<std::path::PathBuf>,
}

impl Session {
    /// Create a new session via the transport
    pub(crate) async fn new(transport: Arc<Transport>, config: SessionConfig) -> Result<Self> {
        // Register tool.call handler before creating the session
        let tool_handlers: Arc<Mutex<HashMap<String, ToolHandler>>> =
            Arc::new(Mutex::new(HashMap::new()));

        let handlers_clone = tool_handlers.clone();
        transport
            .set_request_handler(
                "tool.call",
                Box::new(move |params: Value| {
                    let handlers = handlers_clone.clone();
                    Box::pin(async move {
                        let req: ToolCallRequest = serde_json::from_value(params)
                            .context("Invalid tool.call params")?;

                        let tool_name = req.tool_name.clone();
                        // mark_unchanged is signalled cleanly by ai.rs — skip tool log
                        if tool_name != "mark_unchanged" {
                            let log_summary = if tool_name == "analyze" {
                                req.arguments.get("intent").and_then(|v| v.as_str())
                                    .unwrap_or("(no intent)").to_string()
                            } else {
                                summarize_args(&req.arguments)
                            };
                            eprintln!("  ⚙  {} {}", tool_name, log_summary);
                        }

                        let handler = {
                            let handlers_lock = handlers.lock().await;
                            handlers_lock.get(&req.tool_name).cloned()
                        };

                        let result = if let Some(handler) = handler {
                            // Run handler on a blocking thread — tool impls (e.g. Starlark eval)
                            // may be CPU-bound and must not block the async executor.
                            let args = req.arguments;
                            tokio::task::spawn_blocking(move || handler(args))
                                .await
                                .unwrap_or_else(|e| ToolResult::failure(format!("Tool panicked: {e}")))
                        } else {
                            ToolResult {
                                text_result_for_llm: format!(
                                    "Tool '{}' is not registered.",
                                    req.tool_name
                                ),
                                result_type: "failure".to_string(),
                                error: Some(format!("Unknown tool: {}", req.tool_name)),
                            }
                        };

                        // Log outcome — just the first line of the error to keep it terse
                        if result.result_type == "failure" {
                            let err = result.error.as_deref().unwrap_or("unknown error");
                            let first_line = err.lines().next().unwrap_or(err);
                            eprintln!("    ✗ {}", first_line);
                        }

                        let response = ToolCallResponse { result };
                        Ok(serde_json::to_value(response)?)
                    })
                }),
            )
            .await;

        // Interactive permission handler — prompts the user for approval
        transport
            .set_request_handler(
                "permission.request",
                Box::new(|params: Value| {
                    Box::pin(async move {
                        // Extract details about the permission request
                        let request = params.get("permissionRequest")
                            .or_else(|| params.get("request"))
                            .cloned()
                            .unwrap_or(params.clone());

                        let tool_name = request.get("toolName")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown");
                        let input = request.get("input")
                            .and_then(|v| v.as_str())
                            .or_else(|| request.get("arguments").and_then(|v| v.as_str()))
                            .unwrap_or("");
                        let description = request.get("description")
                            .and_then(|v| v.as_str())
                            .unwrap_or("");

                        // Display what's being requested
                        if !description.is_empty() {
                            eprintln!("  🔐 Permission: {} — {}", tool_name, description);
                        } else if !input.is_empty() {
                            let summary = if input.len() > 120 { &input[..120] } else { input };
                            eprintln!("  🔐 Permission: {} — {}", tool_name, summary);
                        } else {
                            eprintln!("  🔐 Permission: {} — {:?}",
                                tool_name,
                                serde_json::to_string(&request).unwrap_or_default()
                            );
                        }

                        // Read y/n from stdin
                        eprint!("     Allow? [y/N] ");
                        let mut line = String::new();
                        let approved = match std::io::stdin().read_line(&mut line) {
                            Ok(_) => line.trim().eq_ignore_ascii_case("y"),
                            Err(_) => false,
                        };

                        let result = if approved {
                            eprintln!("     \x1b[32m✓\x1b[0m Approved");
                            PermissionResult::approve()
                        } else {
                            eprintln!("     ✗ Denied");
                            PermissionResult::deny()
                        };

                        let response = PermissionResponse { result };
                        Ok(serde_json::to_value(response)?)
                    })
                }),
            )
            .await;

        // Create the session
        let params = serde_json::to_value(&config)?;
        let result = transport
            .request("session.create", params)
            .await
            .context("Failed to create session")?;

        let resp: CreateSessionResponse = serde_json::from_value(result)?;
        tracing::debug!("Created session: {}", resp.session_id);

        Ok(Session {
            transport,
            session_id: resp.session_id,
            tool_handlers,
            root: config.working_directory.as_ref().map(std::path::PathBuf::from),
        })
    }

    /// Register a tool handler for this session
    pub async fn register_tool(&self, name: &str, handler: ToolHandler) {
        let mut handlers = self.tool_handlers.lock().await;
        handlers.insert(name.to_string(), handler);
    }

    /// Send a message and wait for the AI to finish (collects the last assistant message)
    pub async fn send_and_wait(&self, prompt: &str) -> Result<String> {
        // Subscribe to session events
        let mut events_rx = self
            .transport
            .subscribe_notifications("session.event")
            .await;

        // Send the prompt
        let send_req = SessionSendRequest {
            session_id: self.session_id.clone(),
            prompt: prompt.to_string(),
        };
        let params = serde_json::to_value(&send_req)?;
        self.transport
            .request("session.send", params)
            .await
            .context("Failed to send message")?;

        // Collect streamed content and wait for completion.
        // We track the latest assistant message — each new `content` field
        // replaces the buffer so intermediate "thinking" messages don't leak
        // into the final answer.
        let mut streamed_message = String::new();
        // Buffer pending tool log lines: emit on success, discard on failure/rejection
        let mut pending_tool_logs: std::collections::HashMap<String, std::collections::VecDeque<String>> = std::collections::HashMap::new();

        loop {
            let event_params = events_rx
                .recv()
                .await
                .ok_or_else(|| anyhow::anyhow!("Event channel closed"))?;

            let notification: SessionEventNotification =
                match serde_json::from_value(event_params.clone()) {
                    Ok(n) => n,
                    Err(e) => {
                        tracing::trace!("Failed to deserialize session event: {} — raw: {}", e, event_params);
                        continue;
                    }
                };

            // Only handle events for our session
            if notification.session_id != self.session_id {
                continue;
            }

            match notification.event.event_type.as_str() {
                ASSISTANT_MESSAGE => {
                    if let Some(content) = &notification.event.data.content {
                        // A `content` field signals a new (or replacement) message —
                        // reset the buffer so we don't carry forward previous turns.
                        streamed_message = content.clone();
                    }
                    if let Some(delta) = &notification.event.data.delta_content {
                        streamed_message.push_str(delta);
                    }
                }
                "tool.execution_start" => {
                    let data = &notification.event.data;
                    let name = data.tool_name.as_deref()
                        .or_else(|| data.extra.get("toolName").and_then(|v| v.as_str()))
                        .or_else(|| data.extra.get("name").and_then(|v| v.as_str()));
                    match name {
                        Some(n) if n == "report_intent" => {} // internal, skip
                        Some(n) => {
                            let summary = summarize_tool_args(n, &data.extra, self.root.as_deref());
                            let line = if summary.is_empty() {
                                format!("  ⚙  {}", n)
                            } else {
                                format!("  ⚙  {} {}", n, summary)
                            };
                            // Buffer — only print once we know the call succeeded
                            pending_tool_logs.entry(n.to_string()).or_default().push_back(line);
                        }
                        None => tracing::debug!("tool.execution_start with unknown keys: {:?}", data.extra.keys().collect::<Vec<_>>()),
                    }
                }
                "tool.execution_complete" => {
                    let data = &notification.event.data;
                    let name = data.tool_name.as_deref()
                        .or_else(|| data.extra.get("toolName").and_then(|v| v.as_str()))
                        .unwrap_or("tool");
                    tracing::debug!("tool.execution_complete: name={} extra_keys={:?}", name, data.extra.keys().collect::<Vec<_>>());
                    if let Some(err) = data.extra.get("error").and_then(|v| v.as_str()) {
                        // Call was rejected/failed — discard the buffered log line silently
                        if let Some(q) = pending_tool_logs.get_mut(name) { q.pop_front(); }
                        tracing::debug!("  ✗  {} rejected: {}", name, err);
                    } else {
                        // Success — emit the buffered log line
                        if let Some(line) = pending_tool_logs.get_mut(name).and_then(|q| q.pop_front()) {
                            eprintln!("{}", line);
                        } else {
                            tracing::debug!("tool.execution_complete for '{}' had no buffered log line", name);
                        }
                    }
                }
                "tool.start" | "tool.call" => {
                    match notification.event.data.tool_name.as_deref() {
                        Some(n) if n == "report_intent" => {}
                        Some(n) => eprintln!("  ⚙  {}", n),
                        None => tracing::debug!("tool.call with no name: {:?}", notification.event.data.extra),
                    }
                }
                "tool.end" | "tool.result" => {
                    tracing::debug!("  ✓ tool completed");
                }
                SESSION_IDLE => {
                    tracing::debug!("Session {} completed ({})", self.session_id, notification.event.event_type);
                    // Flush any pending tool logs that never got a completion event
                    for lines in pending_tool_logs.values_mut() {
                        for line in lines.drain(..) {
                            eprintln!("{}", line);
                        }
                    }
                    break;
                }
                ASSISTANT_TURN_END => {
                    tracing::debug!("Turn ended in session {}", self.session_id);
                    // Don't break — more turns may follow (tool calls then final answer)
                }
                SESSION_ERROR => {
                    let msg = notification
                        .event
                        .data
                        .message
                        .unwrap_or_else(|| "Unknown session error".to_string());
                    anyhow::bail!("Session error: {}", msg);
                }
                other => {
                    tracing::debug!("Unhandled event: {} data keys: {:?}", other, notification.event.data.extra.keys().collect::<Vec<_>>());
                }
            }
        }

        // Prefer getMessages for the final answer — it reliably returns only
        // the last assistant message without any intermediate thinking text.
        match self.get_last_assistant_message().await {
            Ok(msg) if !msg.is_empty() => {
                tracing::debug!("Using getMessages content ({} bytes)", msg.len());
                Ok(msg)
            }
            _ => {
                // Fall back to streamed content if getMessages is unavailable
                tracing::debug!("Using streamed content ({} bytes)", streamed_message.len());
                Ok(streamed_message)
            }
        }
    }

    /// Fetch conversation history and extract the last assistant message content
    async fn get_last_assistant_message(&self) -> Result<String> {
        let params = serde_json::json!({ "sessionId": self.session_id });
        let result = self
            .transport
            .request("session.getMessages", params)
            .await
            .context("Failed to get session messages")?;

        // The response contains an array of events; find the last assistant.message
        let events = result
            .get("events")
            .and_then(|e| e.as_array())
            .cloned()
            .unwrap_or_default();

        for event in events.iter().rev() {
            let event_type = event
                .get("type")
                .and_then(|t| t.as_str())
                .unwrap_or("");
            if event_type == ASSISTANT_MESSAGE {
                if let Some(content) = event
                    .get("data")
                    .and_then(|d| d.get("content"))
                    .and_then(|c| c.as_str())
                {
                    tracing::debug!("Retrieved {} bytes from session.getMessages", content.len());
                    return Ok(content.to_string());
                }
            }
        }

        Ok(String::new())
    }

    /// Destroy this session
    pub async fn destroy(&self) -> Result<()> {
        let params =
            serde_json::json!({ "sessionId": self.session_id });
        self.transport
            .request("session.destroy", params)
            .await
            .context("Failed to destroy session")?;
        Ok(())
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        tracing::debug!("Session {} dropped (not destroyed — call destroy() explicitly)", self.session_id);
    }
}

/// Build a compact one-line summary of tool arguments for logging
fn summarize_args(args: &Value) -> String {
    match args {
        Value::Object(map) => {
            let parts: Vec<String> = map
                .iter()
                .map(|(k, v)| {
                    let val = match v {
                        Value::String(s) => {
                            if s.len() > 60 {
                                format!("\"{}…\"", &s[..57])
                            } else {
                                format!("\"{}\"", s)
                            }
                        }
                        other => {
                            let s = other.to_string();
                            if s.len() > 40 {
                                format!("{}…", &s[..37])
                            } else {
                                s
                            }
                        }
                    };
                    format!("{}={}", k, val)
                })
                .collect();
            parts.join(", ")
        }
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

/// Extract a compact summary of tool arguments from a tool.execution_start event.
/// Paths are made relative to `root` when possible; out-of-root paths (e.g. /tmp/) are suppressed.
fn summarize_tool_args(tool_name: &str, extra: &serde_json::Map<String, Value>, root: Option<&std::path::Path>) -> String {
    let args = match extra.get("arguments") {
        Some(Value::Object(map)) => map,
        Some(Value::String(s)) => {
            if let Ok(Value::Object(map)) = serde_json::from_str(s) {
                return format_tool_summary(tool_name, &map, root);
            }
            return String::new();
        }
        _ => return String::new(),
    };
    format_tool_summary(tool_name, args, root)
}

/// Returns None if the path is outside the root (e.g. /tmp), Some(relative) if inside.
fn relativize_path<'a>(path: &'a str, root: Option<&std::path::Path>) -> Option<String> {
    let p = std::path::Path::new(path);
    // Filter out clearly non-workspace paths (temp dirs, system paths)
    let s = path.replace('\\', "/");
    if s.starts_with("/tmp/") || s.starts_with("/var/folders/") || s.contains("AppData\\Local\\Temp") {
        return None; // suppress internal copilot temp files
    }
    if let Some(root) = root {
        if let Ok(rel) = p.strip_prefix(root) {
            let rel_str = rel.to_string_lossy().replace('\\', "/");
            return Some(if rel_str.is_empty() { ".".to_string() } else { rel_str });
        }
    }
    Some(path.to_string())
}

fn format_tool_summary(tool_name: &str, args: &serde_json::Map<String, Value>, root: Option<&std::path::Path>) -> String {
    let s = |key: &str| args.get(key).and_then(|v| v.as_str());
    let path_arg = |key: &str| -> Option<String> {
        s(key).and_then(|p| relativize_path(p, root))
    };
    match tool_name {
        "view" | "read_file" => {
            path_arg("path").unwrap_or_default()
        }
        "grep" | "search_files" => {
            let pattern = s("pattern").or_else(|| s("query")).unwrap_or("?");
            match path_arg("path").or_else(|| path_arg("include")) {
                Some(p) if !p.is_empty() => format!("{:?} in {}", pattern, p),
                _ => format!("{:?}", pattern),
            }
        }
        "powershell" | "run_command" => {
            let cmd = s("command").unwrap_or("?");
            if cmd.len() > 80 { format!("{}…", &cmd[..77]) } else { cmd.to_string() }
        }
        "list_files" | "list_dir" => {
            path_arg("path").unwrap_or_else(|| ".".to_string())
        }
        "glob" => {
            s("pattern").unwrap_or("?").to_string()
        }
        _ => {
            args.values()
                .filter_map(|v| v.as_str())
                .filter_map(|v| relativize_path(v, root))
                .next()
                .map(|v| if v.len() > 60 { format!("{}…", &v[..57]) } else { v })
                .unwrap_or_default()
        }
    }
}
