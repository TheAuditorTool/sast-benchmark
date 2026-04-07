fn get_file_path(_user_input: &str) -> &'static str {
    "/app/static/public.html"
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("f");

    match std::fs::read_to_string(get_file_path(&input)) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
