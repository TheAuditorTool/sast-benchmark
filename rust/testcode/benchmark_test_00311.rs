use fancy_regex::Regex;

struct PatternReq {
    pattern: String,
    text: String,
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let preq = PatternReq {
        pattern: req.param("pattern"),
        text: req.param("text"),
    };

    let re = match Regex::new(&preq.pattern) {
        Ok(r) => r,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };

    let found = re.is_match(&preq.text).unwrap_or(false);
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
