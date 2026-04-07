use serde::Deserialize;

#[derive(Deserialize)]
struct SearchRequest {
    query: String,
    max_results: u32,
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.body_str();
    let sr: SearchRequest = match serde_json::from_str(&body) {
        Ok(s) => s,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };
    if sr.max_results > 100 {
        return super::shared::BenchmarkResponse::bad_request("max_results cannot exceed 100");
    }
    if sr.query.len() > 200 {
        return super::shared::BenchmarkResponse::bad_request("query too long");
    }
    super::shared::BenchmarkResponse::ok(&format!("Searching for '{}' limit {}", sr.query, sr.max_results))
}
