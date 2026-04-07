use regex::Regex;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let text = req.param("text");

    if text.len() >= 20 {
        return super::shared::BenchmarkResponse::bad_request("Input too long");
    }

    let re = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();

    let found = re.is_match(&text);
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
