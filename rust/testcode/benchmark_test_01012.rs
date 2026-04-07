pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_input = req.param("username");

    let sanitized = sanitize_for_log(&user_input);
    eprintln!("[INFO] Login attempt: user={}", sanitized);

    super::shared::BenchmarkResponse::ok("Logged")
}

fn sanitize_for_log(input: &str) -> String {
    input.replace('\n', "\\n").replace('\r', "\\r").replace('\t', "\\t")
}
