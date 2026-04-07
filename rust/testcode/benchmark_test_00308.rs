use regex::Regex;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let text = req.param("username");
    let re = Regex::new(r"^[a-zA-Z0-9]{1,64}$").unwrap();
    let valid = re.is_match(&text);
    super::shared::BenchmarkResponse::ok(&format!("Valid username: {}", valid))
}
