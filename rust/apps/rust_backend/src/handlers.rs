//! HTTP Handlers Module
//!
//! Additional vulnerable HTTP handlers for SAST testing.

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::process::{Command, Stdio};

#[derive(Debug, Deserialize)]
pub struct UploadRequest {
    pub filename: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ShellRequest {
    pub shell: String,
    pub script: String,
}

#[derive(Debug, Deserialize)]
pub struct IncludeRequest {
    pub module_path: String,
}

#[derive(Debug, Serialize)]
pub struct GenericResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

/// Configure additional routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v2")
            .route("/upload", web::post().to(upload_file))
            .route("/shell", web::post().to(shell_exec))
            .route("/include", web::post().to(include_file))
            .route("/env", web::get().to(get_env))
            .route("/headers", web::get().to(echo_headers))
    );
}

// -----------------------------------------------------------------------------
// VULNERABILITY: Path Traversal via file upload
// Source: req.filename (user controlled)
// Sink: File::create()
// -----------------------------------------------------------------------------
// vuln-code-snippet start pathtraverBackendUploadFile
async fn upload_file(req: web::Json<UploadRequest>) -> impl Responder {
    // TAINT SINK: User controlled filename
    // Attacker payload: filename="../../../etc/cron.d/evil"
    let filepath = format!("/tmp/uploads/{}", req.filename); // vuln-code-snippet vuln-line pathtraverBackendUploadFile

    match File::create(&filepath) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(req.content.as_bytes()) {
                return HttpResponse::InternalServerError().json(GenericResponse {
                    success: false,
                    message: format!("Write failed: {}", e),
                    data: None,
                });
            }

            HttpResponse::Ok().json(GenericResponse {
                success: true,
                message: format!("File written to {}", filepath),
                data: Some(serde_json::json!({
                    "path": filepath,
                    "size": req.content.len()
                })),
            })
        }
        Err(e) => HttpResponse::InternalServerError().json(GenericResponse {
            success: false,
            message: format!("Failed to create file: {}", e),
            data: None,
        }),
    }
}
// vuln-code-snippet end pathtraverBackendUploadFile

// -----------------------------------------------------------------------------
// VULNERABILITY: Shell command injection via piped shell
// Source: req.script (user controlled)
// Sink: Command with stdin
// -----------------------------------------------------------------------------
// vuln-code-snippet start cmdiBackendShellExec
async fn shell_exec(req: web::Json<ShellRequest>) -> impl Responder {
    // TAINT SINK: User controlled shell and script
    // Attacker payload: shell="/bin/sh", script="cat /etc/passwd"
    let child = Command::new(&req.shell) // vuln-code-snippet vuln-line cmdiBackendShellExec
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    match child {
        Ok(mut process) => {
            // Write user script to shell stdin
            if let Some(mut stdin) = process.stdin.take() {
                let _ = stdin.write_all(req.script.as_bytes());
            }

            match process.wait_with_output() {
                Ok(output) => HttpResponse::Ok().json(GenericResponse {
                    success: true,
                    message: "Command executed".to_string(),
                    data: Some(serde_json::json!({
                        "stdout": String::from_utf8_lossy(&output.stdout),
                        "stderr": String::from_utf8_lossy(&output.stderr),
                        "exit_code": output.status.code(),
                        "shell": &req.shell
                    })),
                }),
                Err(e) => HttpResponse::InternalServerError().json(GenericResponse {
                    success: false,
                    message: format!("Wait failed: {}", e),
                    data: None,
                }),
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(GenericResponse {
            success: false,
            message: format!("Spawn failed: {}", e),
            data: None,
        }),
    }
}
// vuln-code-snippet end cmdiBackendShellExec

// -----------------------------------------------------------------------------
// VULNERABILITY: Local File Inclusion
// Source: req.module_path (user controlled)
// Sink: File::open() with read
// -----------------------------------------------------------------------------
// vuln-code-snippet start pathtraverBackendIncludeFile
async fn include_file(req: web::Json<IncludeRequest>) -> impl Responder {
    // TAINT SINK: User controlled file path for inclusion
    // Attacker payload: module_path="/etc/passwd"
    let mut content = String::new();

    match File::open(&req.module_path) { // vuln-code-snippet vuln-line pathtraverBackendIncludeFile
        Ok(mut file) => {
            if let Err(e) = file.read_to_string(&mut content) {
                return HttpResponse::InternalServerError().json(GenericResponse {
                    success: false,
                    message: format!("Read failed: {}", e),
                    data: None,
                });
            }

            HttpResponse::Ok().json(GenericResponse {
                success: true,
                message: "File included".to_string(),
                data: Some(serde_json::json!({
                    "path": &req.module_path,
                    "content": content,
                    "size": content.len()
                })),
            })
        }
        Err(e) => HttpResponse::InternalServerError().json(GenericResponse {
            success: false,
            message: format!("Failed to include {}: {}", req.module_path, e),
            data: None,
        }),
    }
}
// vuln-code-snippet end pathtraverBackendIncludeFile

// -----------------------------------------------------------------------------
// VULNERABILITY: Environment variable exposure
// -----------------------------------------------------------------------------
// vuln-code-snippet start infodisclosureBackendGetEnv
async fn get_env() -> impl Responder {
    // VULNERABILITY: Exposing all environment variables
    let env_vars: std::collections::HashMap<String, String> = std::env::vars().collect(); // vuln-code-snippet vuln-line infodisclosureBackendGetEnv

    HttpResponse::Ok().json(GenericResponse {
        success: true,
        message: "Environment variables".to_string(),
        data: Some(serde_json::to_value(env_vars).unwrap_or_default()),
    })
}
// vuln-code-snippet end infodisclosureBackendGetEnv

// -----------------------------------------------------------------------------
// VULNERABILITY: Header reflection (potential XSS in logs/responses)
// -----------------------------------------------------------------------------
async fn echo_headers(req: actix_web::HttpRequest) -> impl Responder {
    // VULNERABILITY: Reflecting user-controlled headers
    let headers: std::collections::HashMap<String, String> = req
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect();

    HttpResponse::Ok().json(GenericResponse {
        success: true,
        message: "Headers reflected".to_string(),
        data: Some(serde_json::to_value(headers).unwrap_or_default()),
    })
}
