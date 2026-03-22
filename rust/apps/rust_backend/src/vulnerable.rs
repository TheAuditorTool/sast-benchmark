//! Vulnerable Code Patterns Module
//!
//! This module demonstrates various vulnerable coding patterns in Rust
//! that SAST tools should detect.

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

// -----------------------------------------------------------------------------
// Shared mutable state for race condition demos
// -----------------------------------------------------------------------------
lazy_static::lazy_static! {
    static ref SHARED_DATA: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref UNSAFE_COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
}

#[derive(Debug, Deserialize)]
pub struct SqlRequest {
    pub query: String,
    pub params: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct RegexRequest {
    pub pattern: String,
    pub input: String,
}

#[derive(Debug, Deserialize)]
pub struct CryptoRequest {
    pub data: String,
    pub key: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct VulnResponse {
    pub success: bool,
    pub result: Option<serde_json::Value>,
    pub vulnerability: String,
}

/// Configure vulnerable routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/vuln")
            .route("/sql", web::post().to(raw_sql))
            .route("/regex", web::post().to(regex_dos))
            .route("/crypto", web::post().to(weak_crypto))
            .route("/race", web::get().to(race_condition))
            .route("/overflow", web::get().to(integer_overflow))
            .route("/format", web::get().to(format_string))
            .route("/deserialize", web::post().to(unsafe_deserialize))
            .route("/memory", web::get().to(memory_corruption))
    );
}

// -----------------------------------------------------------------------------
// VULNERABILITY: SQL Injection via raw query string
// -----------------------------------------------------------------------------
// vuln-code-snippet start sqliBackendRawSql
async fn raw_sql(req: web::Json<SqlRequest>) -> impl Responder {
    // TAINT SINK: User query directly used
    // Attacker payload: query="SELECT * FROM users; DROP TABLE users;--"
    let query = &req.query; // vuln-code-snippet vuln-line sqliBackendRawSql

    // Simulating what would happen with raw SQL
    let simulated_result = format!(
        "Would execute: {}\nWith params: {:?}",
        query,
        req.params
    );

    HttpResponse::Ok().json(VulnResponse {
        success: true,
        result: Some(serde_json::json!({
            "query": query,
            "simulated": simulated_result
        })),
        vulnerability: "SQL Injection via raw query".to_string(),
    })
}
// vuln-code-snippet end sqliBackendRawSql

// -----------------------------------------------------------------------------
// VULNERABILITY: ReDoS (Regular Expression Denial of Service)
// -----------------------------------------------------------------------------
// vuln-code-snippet start redosBackendRegexDos
async fn regex_dos(req: web::Json<RegexRequest>) -> impl Responder {
    use fancy_regex::Regex;

    // TAINT SINK: User controlled regex pattern
    // Attacker payload: pattern="(a+)+$", input="aaaaaaaaaaaaaaaaaaaaaaaa!"
    let pattern = match Regex::new(&req.pattern) { // vuln-code-snippet vuln-line redosBackendRegexDos
        Ok(p) => p,
        Err(e) => {
            return HttpResponse::BadRequest().json(VulnResponse {
                success: false,
                result: None,
                vulnerability: format!("Invalid regex: {}", e),
            });
        }
    };

    // This could hang with catastrophic backtracking
    let is_match = pattern.is_match(&req.input).unwrap_or(false);

    HttpResponse::Ok().json(VulnResponse {
        success: true,
        result: Some(serde_json::json!({
            "pattern": &req.pattern,
            "input": &req.input,
            "matched": is_match
        })),
        vulnerability: "ReDoS - catastrophic backtracking possible".to_string(),
    })
}
// vuln-code-snippet end redosBackendRegexDos

// vuln-code-snippet start redosBackendRegexSafe
///Regex pattern validated for length and dangerous patterns
async fn regex_safe(req: web::Json<RegexRequest>) -> impl Responder {
    use regex::Regex;
    if req.pattern.len() > 256 {
        return HttpResponse::BadRequest().json(VulnResponse {
            success: false, result: None,
            vulnerability: "Pattern too long".to_string(),
        });
    }
    if req.input.len() > 10_000 { // vuln-code-snippet safe-line redosBackendRegexSafe
        return HttpResponse::BadRequest().json(VulnResponse {
            success: false, result: None,
            vulnerability: "Input too long".to_string(),
        });
    }
    let pattern = match Regex::new(&req.pattern) {
        Ok(p) => p,
        Err(e) => return HttpResponse::BadRequest().json(VulnResponse {
            success: false, result: None,
            vulnerability: format!("Invalid regex: {}", e),
        }),
    };
    let is_match = pattern.is_match(&req.input);
    HttpResponse::Ok().json(VulnResponse {
        success: true,
        result: Some(serde_json::json!({"matched": is_match})),
        vulnerability: "ReDoS protected".to_string(),
    })
}
// vuln-code-snippet end redosBackendRegexSafe

// -----------------------------------------------------------------------------
// VULNERABILITY: Weak Cryptography
// -----------------------------------------------------------------------------
// vuln-code-snippet start cryptoBackendWeakCrypto
async fn weak_crypto(req: web::Json<CryptoRequest>) -> impl Responder {
    // VULNERABILITY: Using weak/broken cryptographic practices

    // 1. Hardcoded key
    let key = req.key.as_deref().unwrap_or("hardcoded_secret_key_123");

    // 2. Using MD5 for security (broken)
    let md5_hash = format!("{:x}", md5::compute(&req.data)); // vuln-code-snippet vuln-line cryptoBackendWeakCrypto

    // 3. XOR "encryption" (not encryption at all)
    let xor_encrypted: Vec<u8> = req.data
        .bytes()
        .zip(key.bytes().cycle())
        .map(|(d, k)| d ^ k)
        .collect();

    HttpResponse::Ok().json(VulnResponse {
        success: true,
        result: Some(serde_json::json!({
            "md5_hash": md5_hash,
            "xor_encrypted": base64_encode(&xor_encrypted),
            "key_used": key,
            "warnings": [
                "MD5 is cryptographically broken",
                "XOR is not encryption",
                "Hardcoded keys are insecure"
            ]
        })),
        vulnerability: "Weak cryptography".to_string(),
    })
}
// vuln-code-snippet end cryptoBackendWeakCrypto

// -----------------------------------------------------------------------------
// VULNERABILITY: Race Condition
// -----------------------------------------------------------------------------
async fn race_condition() -> impl Responder {
    // VULNERABILITY: Race condition with shared mutable state

    let mut handles = vec![];

    // Spawn multiple threads that access shared state
    for i in 0..10 {
        let data = SHARED_DATA.clone();
        let handle = thread::spawn(move || {
            // This could cause race conditions
            let mut guard = data.lock().unwrap();
            let key = format!("key_{}", i);
            let value = format!("value_{}", i);
            guard.insert(key.clone(), value);

            // VULNERABILITY: Check-then-act race
            if guard.contains_key(&key) {
                // Another thread could modify between check and use
                guard.get(&key).cloned()
            } else {
                None
            }
        });
        handles.push(handle);
    }

    // Wait for all threads
    let results: Vec<_> = handles.into_iter().map(|h| h.join().ok()).collect();

    HttpResponse::Ok().json(VulnResponse {
        success: true,
        result: Some(serde_json::json!({
            "thread_results": results,
            "final_state_size": SHARED_DATA.lock().unwrap().len()
        })),
        vulnerability: "Race condition with shared state".to_string(),
    })
}

// -----------------------------------------------------------------------------
// VULNERABILITY: Integer Overflow
// -----------------------------------------------------------------------------
// vuln-code-snippet start intoverflowBackendOverflow
async fn integer_overflow(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let a: i32 = query.get("a").and_then(|s| s.parse().ok()).unwrap_or(0);
    let b: i32 = query.get("b").and_then(|s| s.parse().ok()).unwrap_or(0);

    // VULNERABILITY: Unchecked arithmetic can overflow
    // Attacker payload: a=2147483647&b=1 (i32::MAX + 1)

    // These will panic in debug mode, wrap in release
    let add_result = a.wrapping_add(b); // vuln-code-snippet vuln-line intoverflowBackendOverflow
    let mul_result = a.wrapping_mul(b);

    // This one might actually overflow/panic without wrapping
    let checked_add = a.checked_add(b);
    let checked_mul = a.checked_mul(b);

    HttpResponse::Ok().json(VulnResponse {
        success: true,
        result: Some(serde_json::json!({
            "inputs": { "a": a, "b": b },
            "wrapping_add": add_result,
            "wrapping_mul": mul_result,
            "checked_add": checked_add,
            "checked_mul": checked_mul,
            "overflow_would_occur": checked_add.is_none() || checked_mul.is_none()
        })),
        vulnerability: "Integer overflow".to_string(),
    })
}
// vuln-code-snippet end intoverflowBackendOverflow

// vuln-code-snippet start intoverflowBackendCheckedArithmetic
///Checked arithmetic prevents overflow
async fn integer_safe(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let a: i32 = query.get("a").and_then(|s| s.parse().ok()).unwrap_or(0);
    let b: i32 = query.get("b").and_then(|s| s.parse().ok()).unwrap_or(0);
    let checked_add = a.checked_add(b); // vuln-code-snippet safe-line intoverflowBackendCheckedArithmetic
    let checked_mul = a.checked_mul(b);
    HttpResponse::Ok().json(VulnResponse {
        success: true,
        result: Some(serde_json::json!({
            "inputs": {"a": a, "b": b},
            "checked_add": checked_add,
            "checked_mul": checked_mul,
            "overflow_detected": checked_add.is_none() || checked_mul.is_none()
        })),
        vulnerability: "Integer operations checked for overflow".to_string(),
    })
}
// vuln-code-snippet end intoverflowBackendCheckedArithmetic

// -----------------------------------------------------------------------------
// VULNERABILITY: Format String (simulated)
// -----------------------------------------------------------------------------
async fn format_string(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let user_input = query.get("fmt").map(|s| s.as_str()).unwrap_or("default");

    // VULNERABILITY: User input used in format-like operation
    // While Rust's format! is safe, this simulates the vulnerability
    let result = user_input.replace("{}", "[REPLACED]")
        .replace("{:x}", "[HEX]")
        .replace("{:p}", "[PTR]");

    HttpResponse::Ok().json(VulnResponse {
        success: true,
        result: Some(serde_json::json!({
            "input": user_input,
            "processed": result
        })),
        vulnerability: "Format string (simulated)".to_string(),
    })
}

// -----------------------------------------------------------------------------
// VULNERABILITY: Unsafe Deserialization
// -----------------------------------------------------------------------------
// vuln-code-snippet start deserBackendUnsafeDeserialize
async fn unsafe_deserialize(body: web::Bytes) -> impl Responder {
    // VULNERABILITY: Deserializing untrusted data

    // Try to deserialize as JSON (relatively safe)
    let json_result: Result<serde_json::Value, _> = serde_json::from_slice(&body); // vuln-code-snippet vuln-line deserBackendUnsafeDeserialize

    // Simulating what would happen with unsafe deserialization
    HttpResponse::Ok().json(VulnResponse {
        success: json_result.is_ok(),
        result: json_result.ok(),
        vulnerability: "Unsafe deserialization of untrusted data".to_string(),
    })
}
// vuln-code-snippet end deserBackendUnsafeDeserialize

// vuln-code-snippet start deserBackendSafeDeserialize
///Typed deserialization with size limit
async fn safe_deserialize(body: web::Bytes) -> impl Responder {
    #[derive(serde::Deserialize)]
    struct SafePayload { data: Option<String>, action: Option<String> }
    if body.len() > 1_048_576 {
        return HttpResponse::BadRequest().json(VulnResponse {
            success: false, result: None,
            vulnerability: "Payload too large".to_string(),
        });
    }
    let result: Result<SafePayload, _> = serde_json::from_slice(&body); // vuln-code-snippet safe-line deserBackendSafeDeserialize
    match result {
        Ok(payload) => HttpResponse::Ok().json(VulnResponse {
            success: true,
            result: Some(serde_json::json!({"data": payload.data, "action": payload.action})),
            vulnerability: "Safe typed deserialization".to_string(),
        }),
        Err(_) => HttpResponse::BadRequest().json(VulnResponse {
            success: false, result: None,
            vulnerability: "Invalid payload".to_string(),
        }),
    }
}
// vuln-code-snippet end deserBackendSafeDeserialize

// -----------------------------------------------------------------------------
// VULNERABILITY: Memory Corruption (unsafe block)
// -----------------------------------------------------------------------------
// vuln-code-snippet start memsafetyBackendMemoryCorruption
async fn memory_corruption(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let offset: usize = query.get("offset")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    // VULNERABILITY: Unsafe memory access with user-controlled offset
    let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
    // No bounds check — out-of-bounds read when offset >= data.len()
    let result = unsafe {
        Some(*data.get_unchecked(offset)) // vuln-code-snippet vuln-line memsafetyBackendMemoryCorruption
    };

    HttpResponse::Ok().json(VulnResponse {
        success: true,
        result: Some(serde_json::json!({
            "offset": offset,
            "data_len": data.len(),
            "value": result,
            "would_be_oob": offset >= data.len()
        })),
        vulnerability: "Memory corruption via unchecked access".to_string(),
    })
}
// vuln-code-snippet end memsafetyBackendMemoryCorruption

// Helper: Simple base64 encoding
fn base64_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();

    for chunk in data.chunks(3) {
        let n = chunk.len();
        let b0 = chunk[0] as usize;
        let b1 = if n > 1 { chunk[1] as usize } else { 0 };
        let b2 = if n > 2 { chunk[2] as usize } else { 0 };

        result.push(ALPHABET[(b0 >> 2)] as char);
        result.push(ALPHABET[((b0 & 0x03) << 4) | (b1 >> 4)] as char);

        if n > 1 {
            result.push(ALPHABET[((b1 & 0x0F) << 2) | (b2 >> 6)] as char);
        } else {
            result.push('=');
        }

        if n > 2 {
            result.push(ALPHABET[b2 & 0x3F] as char);
        } else {
            result.push('=');
        }
    }

    result
}

// MD5 implementation stub (would use a crate in real code)
mod md5 {
    pub struct Digest([u8; 16]);

    impl std::fmt::LowerHex for Digest {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for byte in &self.0 {
                write!(f, "{:02x}", byte)?;
            }
            Ok(())
        }
    }

    pub fn compute(data: &str) -> Digest {
        // Simplified/fake MD5 for demo (real impl would be much more complex)
        let mut hash = [0u8; 16];
        for (i, byte) in data.bytes().enumerate() {
            hash[i % 16] ^= byte;
        }
        Digest(hash)
    }
}
