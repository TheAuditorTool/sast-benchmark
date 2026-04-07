fn delete_account(user_id: &str) -> bool {
    let _ = user_id;
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("user_id");
    let result = delete_account(&user_id);
    if result {
        super::shared::BenchmarkResponse::ok("account deleted")
    } else {
        super::shared::BenchmarkResponse::error("deletion failed")
    }
}
