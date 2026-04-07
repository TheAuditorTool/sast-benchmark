use regex::Regex;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("email");
    if input.len() > 254 {
        return super::shared::BenchmarkResponse::bad_request("Input too long");
    }
    let pattern = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    if pattern.is_match(&input) {
        super::shared::BenchmarkResponse::ok("Valid email")
    } else {
        super::shared::BenchmarkResponse::bad_request("Invalid email")
    }
}
