use fancy_regex::Regex;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let text = req.param("text");
    if text.len() > 256 {
        return super::shared::BenchmarkResponse::bad_request("Input too long");
    }
    let re = Regex::new(r"(a+)+$").unwrap();
    let found = re.is_match(&text).unwrap_or(false);
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
