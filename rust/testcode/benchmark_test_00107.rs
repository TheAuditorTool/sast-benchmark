pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let action = req.param("action");

    log_action(&username, &action);

    super::shared::BenchmarkResponse::ok("Logged")
}

fn log_action(user: &str, action: &str) {
    eprintln!("[ACTION] user={} action={}", user, action);
}
