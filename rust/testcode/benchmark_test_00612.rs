fn delete_account(user_id: &str) -> bool {
    let _ = user_id;
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    if req.header("Origin") != "https://app.example.com" {
        return super::shared::BenchmarkResponse::forbidden("invalid origin");
    }
    let user_id = req.param("user_id");
    let result = delete_account(&user_id);
    if result {
        super::shared::BenchmarkResponse::ok("account deleted")
    } else {
        super::shared::BenchmarkResponse::error("deletion failed")
    }
}
