//! CWE-200: Internal server IP address assembled via format! and returned in response.

// vuln-code-snippet start testcodeInfodisclosure029
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _id = req.param("id");
    let internal_ip = get_internal_ip();
    let msg = format!("Server: {} processed your request", internal_ip); // vuln-code-snippet target-line testcodeInfodisclosure029
    super::shared::BenchmarkResponse::ok(&msg)
}

fn get_internal_ip() -> &'static str { "192.168.1.100" }
// vuln-code-snippet end testcodeInfodisclosure029
