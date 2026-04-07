use fancy_regex::Regex;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let search_pattern = req.param("search");
    let large_text = "a]".repeat(100_000);

    let re = match Regex::new(&search_pattern) {
        Ok(r) => r,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };

    let found = re.is_match(&large_text).unwrap_or(false);

    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
