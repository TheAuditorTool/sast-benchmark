fn is_within_base(full: &std::path::Path, base: &str) -> bool {
    full.starts_with(base)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let path = req.param("path");
    let base = "/allowed/base";
    let joined = format!("{}/{}", base, path);

    let canonical = match std::fs::canonicalize(&joined) {
        Ok(p) => p,
        Err(e) => return super::shared::BenchmarkResponse::error(&e.to_string()),
    };

    if !is_within_base(&canonical, base) {
        return super::shared::BenchmarkResponse::forbidden("Path traversal blocked");
    }

    match std::fs::read_to_string(&canonical) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
