use fancy_regex::Regex;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let text = req.param("text");
    let re = Regex::new(".*a.*b.*c.*d").unwrap();
    let found = re.is_match(&text).unwrap_or(false);
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
