use fancy_regex::Regex;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let text = req.param("version");
    let re = Regex::new(r"(\d+\.)+\d+").unwrap();
    let found = re.is_match(&text).unwrap_or(false);
    super::shared::BenchmarkResponse::ok(&format!("Valid version: {}", found))
}
