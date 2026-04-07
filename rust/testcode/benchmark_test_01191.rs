use std::fs;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let module = req.param("module");
    let path = format!("/opt/app/modules/{}/config.toml", module);
    match fs::read_to_string(&path) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
