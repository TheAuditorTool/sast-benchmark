use regex::Regex;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let pattern_name = req.param("pattern_name");
    let text = req.param("text");

    let pattern = match pattern_name.as_str() {
        "digits" => r"^\d+$",
        "alpha" => r"^[a-zA-Z]+$",
        "email" => r"^[^@]+@[^@]+\.[^@]+$",
        _ => return super::shared::BenchmarkResponse::bad_request("Unknown pattern name"),
    };

    let re = Regex::new(pattern).unwrap();
    let found = re.is_match(&text);

    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
