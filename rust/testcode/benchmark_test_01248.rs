use regex::Regex;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("data");
    let pattern = Regex::new(r"^(a+)+$").unwrap();
    if pattern.is_match(&input) {
        super::shared::BenchmarkResponse::ok("Matched")
    } else {
        super::shared::BenchmarkResponse::ok("No match")
    }
}
