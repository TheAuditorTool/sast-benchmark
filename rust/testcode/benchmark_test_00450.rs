fn create_admin_account(username: &str) -> String {
    format!("admin_account_created_for_{}", username)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let result = create_admin_account(&username);
    super::shared::BenchmarkResponse::ok(&result)
}
