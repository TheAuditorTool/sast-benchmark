pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw: u32 = match req.param("code").parse() {
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Invalid code"),
    };

    let ch: char = match char::try_from(raw) {
        Ok(c) => c,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Invalid char code"),
    };

    super::shared::BenchmarkResponse::ok(&format!("Char: {}", ch))
}
