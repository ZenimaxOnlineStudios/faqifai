//! Copilot CLI client — spawns and manages the copilot process.

use super::jsonrpc::Transport;
use super::session::Session;
use super::types::*;
use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::process::Command;

/// Options for creating a Copilot client
#[derive(Debug, Clone, Default)]
pub struct ClientOptions {
    /// Path to copilot CLI binary. If None, searches COPILOT_CLI_PATH env then PATH.
    pub cli_path: Option<String>,
    /// Working directory for the copilot process.
    /// Determines which workspace skills, agents, and configs copilot discovers.
    pub working_directory: Option<std::path::PathBuf>,
}

/// The Copilot SDK client. Manages the copilot process and sessions.
pub struct Client {
    transport: Arc<Transport>,
    _child: tokio::process::Child,
}

impl Client {
    /// Create a new client by spawning the copilot CLI process
    pub async fn new(options: ClientOptions) -> Result<Self> {
        let cli_path = resolve_cli_path(&options)?;

        tracing::info!("Spawning copilot CLI: {}", cli_path);

        let mut cmd = Command::new(&cli_path);
        cmd.args(["--headless", "--no-auto-update", "--log-level", "info", "--stdio"])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .kill_on_drop(true);

        if let Some(ref cwd) = options.working_directory {
            cmd.current_dir(cwd);
        }

        let mut child = cmd
            .spawn()
            .with_context(|| format!("Failed to spawn copilot CLI at '{}'", cli_path))?;

        let stdin = child.stdin.take().expect("stdin must be piped");
        let stdout = child.stdout.take().expect("stdout must be piped");

        let transport = Arc::new(Transport::new(stdin, stdout));

        let client = Client {
            transport,
            _child: child,
        };

        // Verify connection with ping
        client.ping().await.context("Failed to ping copilot CLI")?;

        Ok(client)
    }

    /// Ping the copilot CLI to verify the connection
    pub async fn ping(&self) -> Result<PingResponse> {
        let params = serde_json::to_value(PingRequest {
            message: "hello".to_string(),
        })?;

        let result = self.transport.request("ping", params).await?;
        let response: PingResponse = serde_json::from_value(result)?;

        if let Some(version) = response.protocol_version {
            if version != SDK_PROTOCOL_VERSION {
                tracing::warn!(
                    "Protocol version mismatch: server={}, sdk={}",
                    version,
                    SDK_PROTOCOL_VERSION
                );
            }
        }

        Ok(response)
    }

    /// Create a new session with the given configuration
    pub async fn create_session(&self, config: SessionConfig) -> Result<Session> {
        Session::new(self.transport.clone(), config).await
    }
}

/// Resolve the path to the copilot CLI binary
fn resolve_cli_path(options: &ClientOptions) -> Result<String> {
    if let Some(ref path) = options.cli_path {
        return Ok(path.clone());
    }

    if let Ok(path) = std::env::var("COPILOT_CLI_PATH") {
        return Ok(path);
    }

    // Default: assume "copilot" is in PATH
    Ok("copilot".to_string())
}
