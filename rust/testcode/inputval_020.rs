//! CWE-20: Enum deserialization via serde rejecting unknown variants automatically.

// vuln-code-snippet start testcodeInputval020
enum Action { Create, Read, Update, Delete }

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let action_str = req.param("action");
    let action = match action_str.as_str() { // vuln-code-snippet target-line testcodeInputval020
        "create" => Action::Create,
        "read" => Action::Read,
        "update" => Action::Update,
        "delete" => Action::Delete,
        _ => return super::shared::BenchmarkResponse::bad_request("Unknown action"),
    };
    let _ = action;
    super::shared::BenchmarkResponse::ok(&format!("Action: {}", action_str))
}
// vuln-code-snippet end testcodeInputval020
