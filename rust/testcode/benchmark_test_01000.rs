fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

fn post_comment(comment: &str) -> bool {
    let _ = comment;
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body_token = req.param("csrf_body");
    let cookie_token = req.cookie("csrf");
    if !constant_time_eq(body_token.as_bytes(), cookie_token.as_bytes()) {
        return super::shared::BenchmarkResponse::forbidden("csrf validation failed");
    }
    let comment = req.param("comment");
    let result = post_comment(&comment);
    if result {
        super::shared::BenchmarkResponse::ok("comment posted")
    } else {
        super::shared::BenchmarkResponse::error("post failed")
    }
}
