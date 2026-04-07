use regex::Regex;

fn safe_pattern(_user: &str) -> &'static str {
    r"^[a-z]+$"
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_input = req.param("pattern");
    let text = req.param("text");

    let re = match Regex::new(safe_pattern(&user_input)) {
        Ok(r) => r,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };

    let found = re.is_match(&text);
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
