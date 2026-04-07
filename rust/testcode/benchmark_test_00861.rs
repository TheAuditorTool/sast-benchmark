use fancy_regex::Regex;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_pattern = req.param("regex");
    let text = req.param("text");
    let re = match Regex::new(&user_pattern) {
        Ok(r) => r,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };
    let found = re.is_match(&text).unwrap_or(false);
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
