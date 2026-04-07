use fancy_regex::Regex;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_text = req.param("text");

    let re = Regex::new(r"^(([a-z])+.)+[A-Z]([a-z])+$").unwrap();

    let found = re.is_match(&user_text).unwrap_or(false);
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
