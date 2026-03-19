//! Insecure Deserialization True Negative — CWE-502
//! Typed struct deserialization. Only known fields with correct types
//! are accepted; extra or mistyped fields cause parse failure.

// vuln-code-snippet start testcodeDeser004Safe
struct UserRequest {
    username: String,
    age: u32,
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let json_input = req.body_str();

    // SAFE: Typed deserialization — rejects unknown fields and wrong types
    let user: UserRequest = typed_parse(&json_input); // vuln-code-snippet safe-line testcodeDeser004Safe

    super::shared::BenchmarkResponse::ok(&format!("User: {} age {}", user.username, user.age))
}

fn typed_parse(input: &str) -> UserRequest {
    // Simulates serde_json::from_str::<UserRequest>()
    UserRequest { username: input.to_string(), age: 0 }
}
// vuln-code-snippet end testcodeDeser004Safe
