//! JSON-RPC 2.0 transport over stdio with Content-Length framing.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::process::{ChildStdin, ChildStdout};
use tokio::sync::{mpsc, oneshot, Mutex};

/// A JSON-RPC 2.0 request/notification
#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Value>,
    pub method: String,
    pub params: Value,
}

/// A JSON-RPC 2.0 response
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<RpcError>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RpcError {
    pub code: i64,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl std::fmt::Display for RpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JSON-RPC Error {}: {}", self.code, self.message)
    }
}

impl std::error::Error for RpcError {}

/// Handler for incoming server→client requests (tool calls, permission requests)
pub type RequestHandlerFn =
    Box<dyn Fn(Value) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Value>> + Send>> + Send + Sync>;

/// The JSON-RPC transport layer
pub struct Transport {
    stdin: Arc<Mutex<ChildStdin>>,
    pending: Arc<Mutex<HashMap<String, oneshot::Sender<Response>>>>,
    request_handlers: Arc<Mutex<HashMap<String, RequestHandlerFn>>>,
    notification_subs: Arc<Mutex<HashMap<String, Vec<mpsc::UnboundedSender<Value>>>>>,
    _read_task: tokio::task::JoinHandle<()>,
}

impl Transport {
    /// Create a new transport from child process stdio handles
    pub fn new(stdin: ChildStdin, stdout: ChildStdout) -> Self {
        let stdin = Arc::new(Mutex::new(stdin));
        let pending: Arc<Mutex<HashMap<String, oneshot::Sender<Response>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let request_handlers: Arc<Mutex<HashMap<String, RequestHandlerFn>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let notification_subs: Arc<Mutex<HashMap<String, Vec<mpsc::UnboundedSender<Value>>>>> =
            Arc::new(Mutex::new(HashMap::new()));

        let read_task = {
            let pending = pending.clone();
            let request_handlers = request_handlers.clone();
            let notification_subs = notification_subs.clone();
            let stdin_for_responses = stdin.clone();

            tokio::spawn(async move {
                if let Err(e) = read_loop(
                    stdout,
                    pending,
                    request_handlers,
                    notification_subs,
                    stdin_for_responses,
                )
                .await
                {
                    tracing::debug!("JSON-RPC read loop ended: {}", e);
                }
            })
        };

        Transport {
            stdin,
            pending,
            request_handlers,
            notification_subs,
            _read_task: read_task,
        }
    }

    /// Send a request and wait for the response
    pub async fn request(&self, method: &str, params: Value) -> Result<Value> {
        let id = uuid::Uuid::new_v4().to_string();
        let (tx, rx) = oneshot::channel();

        {
            let mut pending = self.pending.lock().await;
            pending.insert(id.clone(), tx);
        }

        let request = Request {
            jsonrpc: "2.0".to_string(),
            id: Some(Value::String(id.clone())),
            method: method.to_string(),
            params,
        };

        self.send_message(&request).await?;

        let response = rx.await.map_err(|_| anyhow::anyhow!("response channel closed"))?;

        // Clean up
        {
            let mut pending = self.pending.lock().await;
            pending.remove(&id);
        }

        if let Some(err) = response.error {
            anyhow::bail!("{}", err);
        }

        Ok(response.result.unwrap_or(Value::Null))
    }

    /// Register a handler for incoming server→client requests
    pub async fn set_request_handler(&self, method: &str, handler: RequestHandlerFn) {
        let mut handlers = self.request_handlers.lock().await;
        handlers.insert(method.to_string(), handler);
    }

    /// Subscribe to notifications for a given method (supports multiple subscribers)
    pub async fn subscribe_notifications(&self, method: &str) -> mpsc::UnboundedReceiver<Value> {
        let (tx, rx) = mpsc::unbounded_channel();
        let mut subs = self.notification_subs.lock().await;
        subs.entry(method.to_string()).or_default().push(tx);
        rx
    }

    /// Send a framed JSON-RPC message
    async fn send_message(&self, message: &impl Serialize) -> Result<()> {
        let data = serde_json::to_vec(message)?;
        let header = format!("Content-Length: {}\r\n\r\n", data.len());

        let mut stdin = self.stdin.lock().await;
        stdin.write_all(header.as_bytes()).await?;
        stdin.write_all(&data).await?;
        stdin.flush().await?;

        Ok(())
    }

    /// Send a JSON-RPC response back to the server (for tool.call, permission.request, etc.)
    #[allow(dead_code)]
    pub async fn send_response(&self, id: Value, result: Result<Value>) -> Result<()> {
        let response = match result {
            Ok(val) => Response {
                jsonrpc: "2.0".to_string(),
                id: Some(id),
                result: Some(val),
                error: None,
            },
            Err(e) => Response {
                jsonrpc: "2.0".to_string(),
                id: Some(id),
                result: None,
                error: Some(RpcError {
                    code: -32603,
                    message: e.to_string(),
                    data: None,
                }),
            },
        };

        self.send_message(&response).await
    }
}

/// Read loop: reads Content-Length framed JSON-RPC messages from stdout
async fn read_loop(
    stdout: ChildStdout,
    pending: Arc<Mutex<HashMap<String, oneshot::Sender<Response>>>>,
    request_handlers: Arc<Mutex<HashMap<String, RequestHandlerFn>>>,
    notification_subs: Arc<Mutex<HashMap<String, Vec<mpsc::UnboundedSender<Value>>>>>,
    stdin: Arc<Mutex<ChildStdin>>,
) -> Result<()> {
    let mut reader = BufReader::new(stdout);
    let mut header_line = String::new();

    loop {
        // Read headers until blank line
        let mut content_length: usize = 0;
        loop {
            header_line.clear();
            let bytes_read = reader.read_line(&mut header_line).await?;
            if bytes_read == 0 {
                return Ok(()); // EOF
            }

            let trimmed = header_line.trim();
            if trimmed.is_empty() {
                break;
            }

            if let Some(len_str) = trimmed.strip_prefix("Content-Length: ") {
                content_length = len_str.parse()?;
            }
        }

        if content_length == 0 {
            continue;
        }

        // Read body
        let mut body = vec![0u8; content_length];
        reader.read_exact(&mut body).await?;

        let raw: Value = serde_json::from_slice(&body)?;

        // Determine message type
        let has_method = raw.get("method").is_some();
        let has_id = raw.get("id").is_some();

        if has_method {
            // It's a request or notification from the server
            let method = raw["method"].as_str().unwrap_or("").to_string();
            let params = raw.get("params").cloned().unwrap_or(Value::Null);

            if has_id {
                // Server→client request (e.g., tool.call) — needs a response
                let id = raw["id"].clone();
                let handlers = request_handlers.lock().await;
                if let Some(handler) = handlers.get(&method) {
                    let result = handler(params).await;
                    let response = match result {
                        Ok(val) => Response {
                            jsonrpc: "2.0".to_string(),
                            id: Some(id),
                            result: Some(val),
                            error: None,
                        },
                        Err(e) => Response {
                            jsonrpc: "2.0".to_string(),
                            id: Some(id),
                            result: None,
                            error: Some(RpcError {
                                code: -32603,
                                message: e.to_string(),
                                data: None,
                            }),
                        },
                    };
                    let data = serde_json::to_vec(&response)?;
                    let header = format!("Content-Length: {}\r\n\r\n", data.len());
                    let mut stdin_lock = stdin.lock().await;
                    stdin_lock.write_all(header.as_bytes()).await?;
                    stdin_lock.write_all(&data).await?;
                    stdin_lock.flush().await?;
                } else {
                    // No request handler — check if notification subscribers want it.
                    // The copilot CLI sends some "notification-like" methods (e.g. session.event)
                    // as requests with an id. Broadcast to subscribers and ack with null.
                    let mut subs = notification_subs.lock().await;
                    let has_subs = if let Some(senders) = subs.get_mut(&method) {
                        senders.retain(|tx| tx.send(params.clone()).is_ok());
                        !senders.is_empty()
                    } else {
                        false
                    };
                    drop(subs);

                    if has_subs {
                        // Acknowledge with null result (like Go SDK's NotificationHandlerFor)
                        let response = Response {
                            jsonrpc: "2.0".to_string(),
                            id: Some(id),
                            result: Some(Value::Null),
                            error: None,
                        };
                        let data = serde_json::to_vec(&response)?;
                        let header = format!("Content-Length: {}\r\n\r\n", data.len());
                        let mut stdin_lock = stdin.lock().await;
                        stdin_lock.write_all(header.as_bytes()).await?;
                        stdin_lock.write_all(&data).await?;
                        stdin_lock.flush().await?;
                    } else {
                        tracing::debug!("No handler for server request: {}", method);
                        // Send method-not-found error
                        let response = Response {
                            jsonrpc: "2.0".to_string(),
                            id: Some(id),
                            result: None,
                            error: Some(RpcError {
                                code: -32601,
                                message: format!("Method not found: {}", method),
                                data: None,
                            }),
                        };
                        let data = serde_json::to_vec(&response)?;
                        let header = format!("Content-Length: {}\r\n\r\n", data.len());
                        let mut stdin_lock = stdin.lock().await;
                        stdin_lock.write_all(header.as_bytes()).await?;
                        stdin_lock.write_all(&data).await?;
                        stdin_lock.flush().await?;
                    }
                }
            } else {
                // Notification — broadcast to all subscribers
                let mut subs = notification_subs.lock().await;
                if let Some(senders) = subs.get_mut(&method) {
                    senders.retain(|tx| tx.send(params.clone()).is_ok());
                }
            }
        } else if has_id {
            // It's a response to one of our requests
            let response: Response = serde_json::from_value(raw)?;
            if let Some(Value::String(id)) = &response.id {
                let mut pending_map = pending.lock().await;
                if let Some(tx) = pending_map.remove(id) {
                    let _ = tx.send(response);
                }
            }
        }
    }
}
