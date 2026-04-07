//! CWE-22: Tainted path stored in struct field then passed to fs::read_to_string().

// vuln-code-snippet start testcodePathtraver020
struct FileReq {
    path: String,
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let req_obj = FileReq { path: req.param("p") };

    match std::fs::read_to_string(&req_obj.path) { // vuln-code-snippet target-line testcodePathtraver020
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver020
