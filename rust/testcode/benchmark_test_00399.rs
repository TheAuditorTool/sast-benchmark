use std::collections::HashMap;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut m: HashMap<&str, String> = HashMap::new();
    m.insert("user", req.param("f"));
    m.insert("safe", "/files/report.txt".to_string());

    let p = m.get("safe").unwrap();
    match std::fs::read_to_string(p) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
