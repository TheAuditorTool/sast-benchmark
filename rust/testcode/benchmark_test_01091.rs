use regex::Regex;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_input = req.param("search");
    let text = req.param("text");

    let escaped = regex::escape(&user_input);

    let re = Regex::new(&escaped).unwrap();
    let found = re.is_match(&text);

    super::shared::BenchmarkResponse::ok(&format!("Match: {}", found))
}
