use regex::Regex;
use std::collections::HashMap;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_pattern = req.param("pattern");
    let text = req.param("text");

    let mut m: HashMap<&str, String> = HashMap::new();
    m.insert("user", user_pattern);
    m.insert("safe", r"^[a-z]+".to_string());

    let p = m.get("safe").unwrap();

    let re = match Regex::new(p) {
        Ok(r) => r,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };

    let found = re.is_match(&text);
    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
