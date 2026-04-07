pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_url = req.param("avatar_url");

    let encoded: String = user_url.bytes().map(|b| match b {
        b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
            String::from(b as char)
        }
        _ => format!("%{:02X}", b),
    }).collect();

    let html = format!("<html><body><img src='{}' alt='avatar'></body></html>", encoded);
    super::shared::BenchmarkResponse::ok(&html)
}
