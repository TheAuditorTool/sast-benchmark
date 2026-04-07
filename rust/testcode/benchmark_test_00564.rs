const APPROVED_URLS: &[&str] = &[
    "https://api.example.com/a",
    "https://cdn.example.com/b",
    "https://static.example.com/c",
];

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let index_str = req.param("index");

    let index: usize = match index_str.parse() {
        Ok(i) => i,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Index must be a number"),
    };

    let url = match APPROVED_URLS.get(index) {
        Some(u) => u,
        None => return super::shared::BenchmarkResponse::bad_request("Index out of range"),
    };

    let _resp = simulated_reqwest_get(url);

    super::shared::BenchmarkResponse::ok(&format!("Fetched: {}", url))
}

fn simulated_reqwest_get(url: &str) -> String {
    format!("Response from {}", url)
}
