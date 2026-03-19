//! Built-in job handlers
//!
//! Common job handlers for typical use cases.
//!
//! INTENTIONAL VULNERABILITIES for SAST testing:
//! - Command injection in ShellHandler
//! - SSRF in HttpHandler
//! - Path traversal in FileHandler

use async_trait::async_trait;
use std::process::Command;
use std::time::Duration;

use jobqueue_core::{Job, JobExecutor, Result};

/// HTTP request job handler
///
/// VULNERABILITY: SSRF - makes requests to user-controlled URLs
pub struct HttpHandler {
    client: reqwest::Client,
    timeout: Duration,
}

impl HttpHandler {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            timeout: Duration::from_secs(30),
        }
    }

    pub fn with_timeout(timeout: Duration) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(timeout)
                .build()
                .unwrap(),
            timeout,
        }
    }
}

impl Default for HttpHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl JobExecutor for HttpHandler {
    fn job_type(&self) -> &str {
        "http_request"
    }

    async fn execute(&self, job: &Job) -> Result<serde_json::Value> {
        let url: String = job.payload.deserialize::<HttpJobPayload>()
            .map_err(|e| jobqueue_core::JobQueueError::SerializationError(e))?
            .url;

        // TAINT SINK: SSRF vulnerability
        // User-controlled URL is fetched without validation
        // Attacker can use: http://169.254.169.254/latest/meta-data/
        let response = self.client
            .get(&url)
            .timeout(self.timeout)
            .send()
            .await
            .map_err(|e| jobqueue_core::JobQueueError::execution("HTTP request failed", e))?;

        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();

        Ok(serde_json::json!({
            "status": status,
            "body": body.chars().take(10000).collect::<String>(),
            "url": url
        }))
    }
}

#[derive(serde::Deserialize)]
struct HttpJobPayload {
    url: String,
    #[serde(default)]
    method: String,
    #[serde(default)]
    headers: std::collections::HashMap<String, String>,
    body: Option<String>,
}

/// Shell command job handler
///
/// VULNERABILITY: Command injection
pub struct ShellHandler {
    allowed_commands: Option<Vec<String>>,
}

impl ShellHandler {
    pub fn new() -> Self {
        Self {
            allowed_commands: None,
        }
    }

    /// Create with allowlist (still vulnerable due to args)
    pub fn with_allowlist(commands: Vec<String>) -> Self {
        Self {
            allowed_commands: Some(commands),
        }
    }
}

impl Default for ShellHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl JobExecutor for ShellHandler {
    fn job_type(&self) -> &str {
        "shell_command"
    }

    async fn execute(&self, job: &Job) -> Result<serde_json::Value> {
        let payload: ShellJobPayload = job.payload.deserialize()
            .map_err(|e| jobqueue_core::JobQueueError::SerializationError(e))?;

        // Weak allowlist check - only checks command name, not args
        if let Some(ref allowed) = self.allowed_commands {
            if !allowed.contains(&payload.command) {
                return Err(jobqueue_core::JobQueueError::PermissionDenied {
                    action: format!("Command '{}' not allowed", payload.command),
                });
            }
        }

        // TAINT SINK: Command injection
        // User-controlled command and args executed directly
        // Attacker payload: command="sh", args=["-c", "cat /etc/passwd"]
        let output = Command::new(&payload.command)
            .args(&payload.args)
            .current_dir(payload.cwd.as_deref().unwrap_or("."))
            .output()
            .map_err(|e| jobqueue_core::JobQueueError::execution("Command execution failed", e))?;

        Ok(serde_json::json!({
            "exit_code": output.status.code(),
            "stdout": String::from_utf8_lossy(&output.stdout),
            "stderr": String::from_utf8_lossy(&output.stderr),
            "command": payload.command,
            "args": payload.args
        }))
    }
}

#[derive(serde::Deserialize)]
struct ShellJobPayload {
    command: String,
    #[serde(default)]
    args: Vec<String>,
    cwd: Option<String>,
}

/// File operation job handler
///
/// VULNERABILITY: Path traversal
pub struct FileHandler {
    base_dir: String,
}

impl FileHandler {
    pub fn new(base_dir: impl Into<String>) -> Self {
        Self {
            base_dir: base_dir.into(),
        }
    }
}

#[async_trait]
impl JobExecutor for FileHandler {
    fn job_type(&self) -> &str {
        "file_operation"
    }

    async fn execute(&self, job: &Job) -> Result<serde_json::Value> {
        let payload: FileJobPayload = job.payload.deserialize()
            .map_err(|e| jobqueue_core::JobQueueError::SerializationError(e))?;

        // TAINT SINK: Path traversal
        // User-controlled path is used without proper sanitization
        // Attacker payload: path="../../../etc/passwd"
        let full_path = format!("{}/{}", self.base_dir, payload.path);

        match payload.operation.as_str() {
            "read" => {
                let content = std::fs::read_to_string(&full_path)
                    .map_err(|e| jobqueue_core::JobQueueError::IoError(e))?;

                Ok(serde_json::json!({
                    "operation": "read",
                    "path": full_path,
                    "content": content,
                    "size": content.len()
                }))
            }
            "write" => {
                let content = payload.content.unwrap_or_default();
                std::fs::write(&full_path, &content)
                    .map_err(|e| jobqueue_core::JobQueueError::IoError(e))?;

                Ok(serde_json::json!({
                    "operation": "write",
                    "path": full_path,
                    "bytes_written": content.len()
                }))
            }
            "delete" => {
                std::fs::remove_file(&full_path)
                    .map_err(|e| jobqueue_core::JobQueueError::IoError(e))?;

                Ok(serde_json::json!({
                    "operation": "delete",
                    "path": full_path
                }))
            }
            "list" => {
                let entries: Vec<String> = std::fs::read_dir(&full_path)
                    .map_err(|e| jobqueue_core::JobQueueError::IoError(e))?
                    .filter_map(|e| e.ok())
                    .map(|e| e.file_name().to_string_lossy().to_string())
                    .collect();

                Ok(serde_json::json!({
                    "operation": "list",
                    "path": full_path,
                    "entries": entries
                }))
            }
            _ => Err(jobqueue_core::JobQueueError::Internal(
                format!("Unknown file operation: {}", payload.operation)
            )),
        }
    }
}

#[derive(serde::Deserialize)]
struct FileJobPayload {
    operation: String,
    path: String,
    content: Option<String>,
}

/// Email job handler (stub)
pub struct EmailHandler {
    smtp_host: String,
    smtp_port: u16,
}

impl EmailHandler {
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self {
            smtp_host: host.into(),
            smtp_port: port,
        }
    }
}

#[async_trait]
impl JobExecutor for EmailHandler {
    fn job_type(&self) -> &str {
        "send_email"
    }

    async fn execute(&self, job: &Job) -> Result<serde_json::Value> {
        let payload: EmailJobPayload = job.payload.deserialize()
            .map_err(|e| jobqueue_core::JobQueueError::SerializationError(e))?;

        // Stub implementation - would actually send email
        tracing::info!(
            to = %payload.to,
            subject = %payload.subject,
            "Sending email (stub)"
        );

        // VULNERABILITY: Logs email content which may be sensitive
        tracing::debug!(body = %payload.body, "Email body");

        Ok(serde_json::json!({
            "sent": true,
            "to": payload.to,
            "subject": payload.subject
        }))
    }
}

#[derive(serde::Deserialize)]
struct EmailJobPayload {
    to: String,
    subject: String,
    body: String,
    #[serde(default)]
    cc: Vec<String>,
    #[serde(default)]
    bcc: Vec<String>,
}

/// Webhook job handler
///
/// VULNERABILITY: SSRF via webhook URL
pub struct WebhookHandler {
    client: reqwest::Client,
}

impl WebhookHandler {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

impl Default for WebhookHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl JobExecutor for WebhookHandler {
    fn job_type(&self) -> &str {
        "webhook"
    }

    async fn execute(&self, job: &Job) -> Result<serde_json::Value> {
        let payload: WebhookJobPayload = job.payload.deserialize()
            .map_err(|e| jobqueue_core::JobQueueError::SerializationError(e))?;

        // TAINT SINK: SSRF - user controls webhook URL
        let response = self.client
            .post(&payload.url)
            .json(&payload.data)
            .send()
            .await
            .map_err(|e| jobqueue_core::JobQueueError::execution("Webhook failed", e))?;

        Ok(serde_json::json!({
            "status": response.status().as_u16(),
            "url": payload.url
        }))
    }
}

#[derive(serde::Deserialize)]
struct WebhookJobPayload {
    url: String,
    data: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_handler_allowlist() {
        let handler = ShellHandler::with_allowlist(vec!["echo".to_string()]);
        assert!(handler.allowed_commands.is_some());
    }
}
