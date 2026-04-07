pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    if username.is_empty() {
        return super::shared::BenchmarkResponse::bad_request("Required");
    }
    let result = create_account(&username);
    super::shared::BenchmarkResponse::ok(&result)
}

fn create_account(name: &str) -> String { format!("created={}", name) }
