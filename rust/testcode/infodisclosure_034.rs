//! CWE-200: Full server binary path and version information exposed in response.

// vuln-code-snippet start testcodeInfodisclosure034
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _id = req.param("id");
    let info = get_server_info();
    super::shared::BenchmarkResponse::ok(&info) // vuln-code-snippet target-line testcodeInfodisclosure034
}

fn get_server_info() -> String {
    "/opt/app/bin/server v1.2.3 built 2024-01-15 on host build-server-01".to_string()
}
// vuln-code-snippet end testcodeInfodisclosure034
