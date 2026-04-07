use std::time::{Duration, Instant};

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_pattern = req.param("pattern");
    let text = req.param("text");

    let re = match fancy_regex::Regex::new(&user_pattern) {
        Ok(r) => r,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };

    let start = Instant::now();
    let found = re.is_match(&text).unwrap_or(false);
    if start.elapsed() > Duration::from_secs(2) {
        return super::shared::BenchmarkResponse::error("Regex execution timed out");
    }

    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
