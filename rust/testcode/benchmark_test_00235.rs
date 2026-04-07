fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

fn change_settings(setting: &str) -> bool {
    let _ = setting;
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body_token = req.param("csrf_body");
    let cookie_token = req.cookie("csrf");
    if !constant_time_eq(body_token.as_bytes(), cookie_token.as_bytes()) {
        return super::shared::BenchmarkResponse::forbidden("csrf validation failed");
    }
    let setting = req.param("setting");
    let result = change_settings(&setting);
    if result {
        super::shared::BenchmarkResponse::ok("settings changed")
    } else {
        super::shared::BenchmarkResponse::error("change failed")
    }
}
