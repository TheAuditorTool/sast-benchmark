use std::collections::HashMap;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut urls = HashMap::new();
    urls.insert("user_url", req.param("url"));
    urls.insert("safe_url", "/dashboard".to_string());
    let dest = urls.get("safe_url").unwrap();
    let location = format!("Location: {}", dest);
    super::shared::BenchmarkResponse::ok(&location)
}
