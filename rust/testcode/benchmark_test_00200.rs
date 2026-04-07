use std::collections::HashMap;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut m: HashMap<&str, String> = HashMap::new();
    m.insert("user_url", req.param("url"));
    m.insert("safe_url", "https://api.example.com/data".to_string());

    let url = m.get("safe_url").unwrap();

    let _resp = simulated_reqwest_get(url);

    super::shared::BenchmarkResponse::ok("Fetched")
}

fn simulated_reqwest_get(url: &str) -> String {
    format!("Response from {}", url)
}
