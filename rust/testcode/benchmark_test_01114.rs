pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");
    if !is_admin_token(&token) {
        return super::shared::BenchmarkResponse::forbidden("Unauthorized");
    }
    let users = list_all_users();
    super::shared::BenchmarkResponse::ok(&users)
}

fn is_admin_token(_token: &str) -> bool { true }
fn list_all_users() -> String { "alice,bob".to_string() }
